use proc_macro::TokenStream;
use quote::quote;
use syn::ItemFn;

/// Extract the inner type from a reference type like &T or &mut T
fn extract_reference_inner_type(param_type: &syn::Type) -> &syn::Type {
    match param_type {
        syn::Type::Reference(type_ref) => &type_ref.elem,
        _ => param_type, // Return as-is if not a reference
    }
}

/// Generates a rustleaf wrapper for a standalone function.
///
/// This takes a function like:
/// ```rust
/// #[rustleaf]
/// fn my_func(param: ParamType) -> ReturnType {
///     // function body
/// }
/// ```
///
/// And generates:
/// ```rust
/// fn my_func(param: ParamType) -> ReturnType {
///     // original function body
/// }
///
/// pub fn rustleaf_my_func(value: rustleaf::core::Value) -> anyhow::Result<rustleaf::core::Value> {
///     let param = ParamType::from_value(value)?;
///     let result = my_func(param);
///     Ok(result.to_value())
/// }
/// ```
pub fn rustleaf_fn(input: ItemFn) -> TokenStream {
    let original_fn = &input;
    let original_name = &input.sig.ident;

    // Extract all parameters
    let params: Vec<_> = input
        .sig
        .inputs
        .iter()
        .map(|param| {
            if let syn::FnArg::Typed(typed_param) = param {
                if let syn::Pat::Ident(ident_pat) = &*typed_param.pat {
                    (ident_pat.ident.clone(), typed_param.ty.clone())
                } else {
                    panic!("Function parameters must be simple identifiers");
                }
            } else {
                panic!("Function parameters must be typed (no self allowed)");
            }
        })
        .collect();

    // Generate wrapper using consistent Vec<Value> interface
    let wrapper_body = generate_wrapper(original_name, &params);

    let expanded = quote! {
        // Keep the original function
        #original_fn

        // Generate the wrapper function
        #wrapper_body
    };

    TokenStream::from(expanded)
}

fn generate_wrapper(
    original_name: &syn::Ident,
    params: &[(syn::Ident, Box<syn::Type>)],
) -> proc_macro2::TokenStream {
    let wrapper_name = quote::format_ident!("rustleaf_{}", original_name);
    let param_count = params.len();

    // Generate parameter conversions by popping from the owned Vec<Value>
    let param_conversions: Vec<_> = params.iter().enumerate().map(|(i, (param_name, param_type))| {
        if is_reference_type(param_type) {
            if is_mutable_reference(param_type) {
                // &mut T -> &mut *value.borrow_mut_value_as()?
                let inner_type = extract_reference_inner_type(param_type);
                quote! {
                    let mut #param_name = values.pop().ok_or_else(|| anyhow::anyhow!("Missing parameter {} for function {}", #i, stringify!(#original_name)))?;
                    let #param_name = &mut *rustleaf::core::BorrowMutValueAs::<#inner_type>::borrow_mut_value_as(&mut #param_name)?;
                }
            } else {
                // &T -> &*value.borrow_value_as()?
                let inner_type = extract_reference_inner_type(param_type);
                quote! {
                    let #param_name = values.pop().ok_or_else(|| anyhow::anyhow!("Missing parameter {} for function {}", #i, stringify!(#original_name)))?;
                    let #param_name = &*rustleaf::core::BorrowValueAs::<#inner_type>::borrow_value_as(&#param_name)?;
                }
            }
        } else {
            // Owned types: T -> T::from_value(value) (no clone needed!)
            quote! {
                let #param_name = values.pop().ok_or_else(|| anyhow::anyhow!("Missing parameter {} for function {}", #i, stringify!(#original_name)))?;
                let #param_name = #param_type::from_value(#param_name)?;
            }
        }
    }).collect();

    // Generate parameter names for function call
    let param_names: Vec<_> = params.iter().map(|(param_name, _)| param_name).collect();

    quote! {
        pub fn #wrapper_name(mut values: Vec<rustleaf::core::Value>) -> anyhow::Result<rustleaf::core::Value> {
            use rustleaf::core::{ToValue, BorrowValueAs, BorrowMutValueAs, FromValue};

            // Check parameter count
            if values.len() != #param_count {
                return Err(anyhow::anyhow!(
                    "Function {} expects {} parameters, got {}",
                    stringify!(#original_name),
                    #param_count,
                    values.len()
                ));
            }

            // Reverse the arguments so we can pop them in the correct order
            values.reverse();

            // Convert each Value to its corresponding Rust type
            #(#param_conversions)*

            // Call the original function
            let result = #original_name(#(#param_names),*);

            // Convert result back to Value using ToValue trait
            Ok(result.to_value())
        }
    }
}

/// Check if a type is a reference type (&T or &mut T)
pub fn is_reference_type(ty: &syn::Type) -> bool {
    matches!(ty, syn::Type::Reference(_))
}

/// Check if a reference type is mutable (&mut T)
pub fn is_mutable_reference(ty: &syn::Type) -> bool {
    if let syn::Type::Reference(type_ref) = ty {
        type_ref.mutability.is_some()
    } else {
        false
    }
}
