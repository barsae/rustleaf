use proc_macro::TokenStream;
use quote::quote;
use syn::{FnArg, ImplItem, ItemImpl, ReturnType};

// Import the systematic type analysis functions from function_wrapper
use crate::function_wrapper::{is_mutable_reference, is_reference_type};

/// Extract the inner type from a reference type like &T or &mut T
fn extract_reference_inner_type(param_type: &syn::Type) -> &syn::Type {
    match param_type {
        syn::Type::Reference(type_ref) => &type_ref.elem,
        _ => param_type, // Return as-is if not a reference
    }
}

/// Convert PascalCase/camelCase to snake_case
/// Example: "SomeUserType" -> "some_user_type", "Point" -> "point"
fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() && i > 0 {
            result.push('_');
        }
        result.push(c.to_ascii_lowercase());
    }
    result
}

/// Generates rustleaf wrappers for methods in an impl block.
///
/// This processes impl blocks like:
/// ```rust
/// #[rustleaf]
/// impl Point {
///     fn new(x: f64, y: f64) -> Self { ... }        // Static method (constructor)
///     fn magnitude(&self) -> f64 { ... }             // Instance method
///     fn distance(&self, other: &Point) -> f64 { ... } // Instance method with parameters  
///     fn scale(&mut self, factor: f64) { ... }       // Mutating method
/// }
/// ```
///
/// And generates:
/// ```rust  
/// impl Point {
///     // Original methods unchanged
/// }
///
/// // Static method wrappers (constructors and static functions) - return PointRef
/// pub fn rustleaf_Point_new(values: Vec<Value>) -> anyhow::Result<Value> { ... }
///
/// // Instance method wrappers on PointRef (self + args in single Vec<Value>)  
/// impl PointRef {
///     pub fn rustleaf_magnitude(values: Vec<Value>) -> anyhow::Result<Value> { ... }
///     pub fn rustleaf_distance(values: Vec<Value>) -> anyhow::Result<Value> { ... }
///     pub fn rustleaf_scale(values: Vec<Value>) -> anyhow::Result<Value> { ... }
/// }
/// ```
pub fn rustleaf_impl(input: ItemImpl) -> TokenStream {
    let original_impl = &input;

    // Extract struct name from impl block
    let struct_name = if let syn::Type::Path(type_path) = &*input.self_ty {
        &type_path.path.segments.last().unwrap().ident
    } else {
        panic!("rustleaf impl blocks must be for named types");
    };

    let mut static_wrappers = Vec::new();
    let mut instance_wrappers = Vec::new();

    // Process each method in the impl block
    for item in &input.items {
        if let ImplItem::Fn(method) = item {
            // Determine if this is a static method (no self parameter) or instance method
            let has_self = method
                .sig
                .inputs
                .iter()
                .any(|arg| matches!(arg, FnArg::Receiver(_)));

            if has_self {
                // Instance method - generates wrapper that takes [self_value, ...args]
                let wrapper = generate_instance_method_wrapper(method, struct_name);
                instance_wrappers.push(wrapper);
            } else {
                // Static method - generates wrapper that takes [...args]
                let wrapper = generate_static_method_wrapper(method, struct_name);
                static_wrappers.push(wrapper);
            }
        }
    }

    let ref_name = quote::format_ident!("{}Ref", struct_name);

    // Generate get_method implementation for method dispatch
    let instance_method_names: Vec<String> = input
        .items
        .iter()
        .filter_map(|item| {
            if let ImplItem::Fn(method) = item {
                // Check if this is an instance method (has self parameter)
                let has_self = method
                    .sig
                    .inputs
                    .iter()
                    .any(|arg| matches!(arg, FnArg::Receiver(_)));
                if has_self {
                    Some(method.sig.ident.to_string())
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    let get_method_impl = generate_get_method_impl(&ref_name, &instance_method_names);

    let expanded = quote! {
        // Keep original impl block unchanged
        #original_impl

        // Generate static method wrappers
        #(#static_wrappers)*

        // Generate instance method wrappers on the Ref type
        impl #ref_name {
            #(#instance_wrappers)*
        }

        // Generate get_method helper
        #get_method_impl
    };

    TokenStream::from(expanded)
}

/// Generate wrapper for static methods (constructors and static functions)
/// Static methods don't have self, so they take Vec<Value> of just the function arguments
fn generate_static_method_wrapper(
    method: &syn::ImplItemFn,
    struct_name: &syn::Ident,
) -> proc_macro2::TokenStream {
    let ref_name = quote::format_ident!("{}Ref", struct_name);
    let method_name = &method.sig.ident;
    let struct_name_snake = to_snake_case(&struct_name.to_string());
    let method_name_snake = to_snake_case(&method_name.to_string());
    let wrapper_name = quote::format_ident!("rustleaf_{}_{}", struct_name_snake, method_name_snake);

    // Extract parameters (no self for static methods)
    let params: Vec<_> = method
        .sig
        .inputs
        .iter()
        .filter_map(|param| {
            if let FnArg::Typed(typed_param) = param {
                if let syn::Pat::Ident(ident_pat) = &*typed_param.pat {
                    Some((ident_pat.ident.clone(), typed_param.ty.clone()))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    let param_count = params.len();

    // Generate parameter conversions using systematic type analysis
    let param_conversions = generate_param_conversions(&params, &wrapper_name, method_name);
    let param_names: Vec<_> = params.iter().map(|(name, _)| name).collect();

    // Check if this returns Self (constructor pattern)
    let returns_self = matches!(
        &method.sig.output,
        ReturnType::Type(_, return_type) if matches!(**return_type, syn::Type::Path(ref path) if path.path.is_ident("Self"))
    );

    if returns_self {
        // Constructor - wrap result in PointRef and then RustValue
        quote! {
            pub fn #wrapper_name(values: Vec<rustleaf::core::Value>) -> anyhow::Result<rustleaf::core::Value> {
                use rustleaf::core::{ToValue, BorrowValueAs, BorrowMutValueAs, FromValue};

                // Check parameter count
                if values.len() != #param_count {
                    return Err(anyhow::anyhow!(
                        "Function {}::{} expects {} parameters, got {}",
                        stringify!(#struct_name),
                        stringify!(#method_name),
                        #param_count,
                        values.len()
                    ));
                }

                // Convert each Value to its corresponding Rust type
                #(#param_conversions)*

                // Call the original static method
                let instance = #struct_name::#method_name(#(#param_names),*);

                // Wrap in PointRef and then as RustValue
                let wrapper = #ref_name::new(instance);
                Ok(rustleaf::core::Value::from_rust(wrapper))
            }
        }
    } else {
        // Regular static method - use ToValue for result conversion
        quote! {
            pub fn #wrapper_name(values: Vec<rustleaf::core::Value>) -> anyhow::Result<rustleaf::core::Value> {
                use rustleaf::core::{ToValue, BorrowValueAs, BorrowMutValueAs, FromValue};

                // Check parameter count
                if values.len() != #param_count {
                    return Err(anyhow::anyhow!(
                        "Function {}::{} expects {} parameters, got {}",
                        stringify!(#struct_name),
                        stringify!(#method_name),
                        #param_count,
                        values.len()
                    ));
                }

                // Convert each Value to its corresponding Rust type
                #(#param_conversions)*

                // Call the original static method
                let result = #struct_name::#method_name(#(#param_names),*);

                // Convert result back to Value using ToValue trait
                Ok(result.to_value())
            }
        }
    }
}

/// Generate wrapper for instance methods  
/// Instance methods take [self_value, ...args] in a single Vec<Value>
/// These are implemented as methods on PointRef
fn generate_instance_method_wrapper(
    method: &syn::ImplItemFn,
    struct_name: &syn::Ident,
) -> proc_macro2::TokenStream {
    let method_name = &method.sig.ident;
    let method_name_snake = to_snake_case(&method_name.to_string());
    let wrapper_name = quote::format_ident!("rustleaf_{}", method_name_snake); // Just method name in snake_case

    // Analyze self parameter and other parameters
    let mut self_mutability = None;
    let mut params = Vec::new();

    for param in &method.sig.inputs {
        match param {
            FnArg::Receiver(receiver) => {
                self_mutability = Some(receiver.mutability.is_some());
            }
            FnArg::Typed(typed_param) => {
                if let syn::Pat::Ident(ident_pat) = &*typed_param.pat {
                    params.push((ident_pat.ident.clone(), typed_param.ty.clone()));
                }
            }
        }
    }

    let is_self_mut = self_mutability.unwrap_or(false);
    let total_param_count = params.len() + 1; // +1 for self

    // Generate parameter conversions using trait-based approach (skip index 0, which is self)
    let param_conversions: Vec<_> = params.iter().enumerate().map(|(i, (param_name, param_type))| {
        let param_index = i + 1; // Skip self at index 0

        if is_reference_type(param_type) {
            if is_mutable_reference(param_type) {
                // &mut T -> &mut *value.borrow_mut_value_as()?
                let inner_type = extract_reference_inner_type(param_type);
                quote! {
                    let #param_name = if let Some(value) = values.get_mut(#param_index) {
                        &mut *rustleaf::core::BorrowMutValueAs::<#inner_type>::borrow_mut_value_as(value)?
                    } else {
                        return Err(anyhow::anyhow!("Missing parameter {} for method {}::{}", #param_index, stringify!(#struct_name), stringify!(#method_name)));
                    };
                }
            } else {
                // &T -> &*value.borrow_value_as()?
                let inner_type = extract_reference_inner_type(param_type);
                quote! {
                    let #param_name = if let Some(value) = values.get(#param_index) {
                        &*rustleaf::core::BorrowValueAs::<#inner_type>::borrow_value_as(value)?
                    } else {
                        return Err(anyhow::anyhow!("Missing parameter {} for method {}::{}", #param_index, stringify!(#struct_name), stringify!(#method_name)));
                    };
                }
            }
        } else {
            // Owned types: T -> T::from_value(value.clone())?
            quote! {
                let #param_name = if let Some(value) = values.get(#param_index) {
                    #param_type::from_value(value.clone())?
                } else {
                    return Err(anyhow::anyhow!("Missing parameter {} for method {}::{}", #param_index, stringify!(#struct_name), stringify!(#method_name)));
                };
            }
        }
    }).collect();

    let param_names: Vec<_> = params.iter().map(|(name, _)| name).collect();

    if is_self_mut {
        // Mutable method - need to downcast and borrow_mut
        quote! {
            pub fn #wrapper_name(values: Vec<rustleaf::core::Value>) -> anyhow::Result<rustleaf::core::Value> {
                use rustleaf::core::{ToValue, BorrowValueAs, BorrowMutValueAs, FromValue};

                // Check parameter count (self + args)
                if values.len() != #total_param_count {
                    return Err(anyhow::anyhow!(
                        "Method {}::{} expects {} parameters (including self), got {}",
                        stringify!(#struct_name),
                        stringify!(#method_name),
                        #total_param_count,
                        values.len()
                    ));
                }

                // Extract self (always at index 0) - should be a PointRef
                let self_value = values.get(0).ok_or_else(|| anyhow::anyhow!("Missing self parameter"))?;
                let self_ref = self_value.downcast_rust_value::<Self>()
                    .ok_or_else(|| anyhow::anyhow!("Expected {}Ref for self parameter, got {}", stringify!(#struct_name), self_value.type_name()))?;

                // Convert other parameters
                #(#param_conversions)*

                // Call the method with mutable borrow through Rc<RefCell<>>
                let mut borrowed = self_ref.borrow_mut();
                let result = borrowed.#method_name(#(#param_names),*);

                // Convert result back to Value
                Ok(result.to_value())
            }
        }
    } else {
        // Immutable method - can use direct reference
        quote! {
            pub fn #wrapper_name(values: Vec<rustleaf::core::Value>) -> anyhow::Result<rustleaf::core::Value> {
                use rustleaf::core::{ToValue, BorrowValueAs, BorrowMutValueAs, FromValue};

                // Check parameter count (self + args)
                if values.len() != #total_param_count {
                    return Err(anyhow::anyhow!(
                        "Method {}::{} expects {} parameters (including self), got {}",
                        stringify!(#struct_name),
                        stringify!(#method_name),
                        #total_param_count,
                        values.len()
                    ));
                }

                // Extract self (always at index 0) - should be a PointRef
                let self_value = values.get(0).ok_or_else(|| anyhow::anyhow!("Missing self parameter"))?;
                let self_ref = self_value.downcast_rust_value::<Self>()
                    .ok_or_else(|| anyhow::anyhow!("Expected {}Ref for self parameter, got {}", stringify!(#struct_name), self_value.type_name()))?;

                // Convert other parameters
                #(#param_conversions)*

                // Call the method with immutable borrow through Rc<RefCell<>>
                let borrowed = self_ref.borrow();
                let result = borrowed.#method_name(#(#param_names),*);

                // Convert result back to Value
                Ok(result.to_value())
            }
        }
    }
}

/// Generate parameter conversions using trait-based approach
fn generate_param_conversions(
    params: &[(syn::Ident, Box<syn::Type>)],
    wrapper_name: &syn::Ident,
    _method_name: &syn::Ident,
) -> Vec<proc_macro2::TokenStream> {
    params.iter().enumerate().map(|(i, (param_name, param_type))| {
        if is_reference_type(param_type) {
            if is_mutable_reference(param_type) {
                // &mut T -> &mut *value.borrow_mut_value_as()?
                let inner_type = extract_reference_inner_type(param_type);
                quote! {
                    let #param_name = if let Some(value) = values.get_mut(#i) {
                        &mut *rustleaf::core::BorrowMutValueAs::<#inner_type>::borrow_mut_value_as(value)?
                    } else {
                        return Err(anyhow::anyhow!("Missing parameter {} for function {}", #i, stringify!(#wrapper_name)));
                    };
                }
            } else {
                // &T -> &*value.borrow_value_as()?
                let inner_type = extract_reference_inner_type(param_type);
                quote! {
                    let #param_name = if let Some(value) = values.get(#i) {
                        &*rustleaf::core::BorrowValueAs::<#inner_type>::borrow_value_as(value)?
                    } else {
                        return Err(anyhow::anyhow!("Missing parameter {} for function {}", #i, stringify!(#wrapper_name)));
                    };
                }
            }
        } else {
            // Owned types: T -> T::from_value(value.clone())?
            quote! {
                let #param_name = if let Some(value) = values.get(#i) {
                    #param_type::from_value(value.clone())?
                } else {
                    return Err(anyhow::anyhow!("Missing parameter {} for function {}", #i, stringify!(#wrapper_name)));
                };
            }
        }
    }).collect()
}

/// Generate get_method implementation for PointRef to support method dispatch
fn generate_get_method_impl(
    ref_name: &syn::Ident,
    method_names: &[String],
) -> proc_macro2::TokenStream {
    let mut method_arms = Vec::new();

    for method_name in method_names {
        let method_name_snake = to_snake_case(method_name);
        let wrapper_name = quote::format_ident!("rustleaf_{}", method_name_snake);

        method_arms.push(quote! {
            #method_name => Some(rustleaf::core::Value::from_rust(rustleaf::core::BoundMethodVec::new(
                &rustleaf::core::Value::rust_value(self.dyn_clone()),
                Self::#wrapper_name,
            ))),
        });
    }

    quote! {
        impl #ref_name {
            /// Generated helper for accessing wrapper methods
            pub fn get_method(&self, name: &str) -> Option<rustleaf::core::Value> {
                match name {
                    #(#method_arms)*
                    _ => None,
                }
            }
        }
    }
}
