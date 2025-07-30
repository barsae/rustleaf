use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, DeriveInput, ImplItem, ImplItemFn, Item, ItemImpl, ItemStruct, ReturnType,
};

#[proc_macro_derive(RustValue)]
pub fn derive_rust_value(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl crate::core::value::RustValue for #name {
            fn type_name(&self) -> &'static str {
                stringify!(#name)
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro]
pub fn rustleaf_tests(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}

#[proc_macro_attribute]
pub fn rust_value_any(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemImpl);

    // Generate the as_any methods
    let as_any_methods = quote! {
        fn as_any(&self) -> &dyn std::any::Any {
            self
        }

        fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
            self
        }
    };

    // Build the new impl block with the added methods
    let ItemImpl {
        attrs,
        defaultness,
        unsafety,
        impl_token,
        generics,
        trait_,
        self_ty,
        brace_token: _,
        items,
    } = input;

    // Insert the as_any methods at the beginning of the impl items
    let expanded = if let Some((bang, path, for_token)) = trait_ {
        quote! {
            #(#attrs)*
            #defaultness #unsafety #impl_token #generics #bang #path #for_token #self_ty {
                #as_any_methods

                #(#items)*
            }
        }
    } else {
        // This shouldn't happen for RustValue impls, but handle it gracefully
        quote! {
            #(#attrs)*
            #defaultness #unsafety #impl_token #generics #self_ty {
                #as_any_methods

                #(#items)*
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(RustLeafWrapper, attributes(core_type))]
pub fn derive_rustleaf_wrapper(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let wrapper_name = &input.ident;

    // Extract the core type from the #[core_type(Type)] attribute
    let core_type = input
        .attrs
        .iter()
        .find_map(|attr| {
            if attr.path().is_ident("core_type") {
                attr.parse_args::<syn::Type>().ok()
            } else {
                None
            }
        })
        .expect("RustLeafWrapper requires #[core_type(Type)] attribute");

    let expanded = quote! {
        impl #wrapper_name {
            pub fn new(inner: #core_type) -> Self {
                Self(std::rc::Rc::new(std::cell::RefCell::new(inner)))
            }

            pub fn borrow(&self) -> std::cell::Ref<#core_type> {
                self.0.borrow()
            }

            pub fn borrow_mut(&self) -> std::cell::RefMut<#core_type> {
                self.0.borrow_mut()
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn rustleaf_methods(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemImpl);

    let mut generated_wrappers = Vec::new();
    let mut method_names = Vec::new();
    let mut filtered_items = Vec::new();

    // Process each method in the impl block
    for item in &input.items {
        if let ImplItem::Fn(method) = item {
            // Check if this method has the #[rustleaf_method] attribute
            let mut is_rustleaf_method = false;
            let mut is_mutating = false;

            for attr in &method.attrs {
                if attr.path().is_ident("rustleaf_method") {
                    is_rustleaf_method = true;

                    // Check for mutating parameter
                    if let Ok(tokens) = attr.parse_args::<syn::Ident>() {
                        if tokens == "mutating" {
                            is_mutating = true;
                        }
                    }
                    break;
                }
            }

            if is_rustleaf_method {
                let wrapper = generate_method_wrapper(method, is_mutating);
                generated_wrappers.push(wrapper);

                let method_name = method.sig.ident.to_string();
                method_names.push(method_name);
            }
        }

        // Keep all items (filtered attributes will be handled by syn)
        filtered_items.push(item.clone());
    }

    // Generate get_attr implementation
    let get_attr_impl = generate_get_attr_impl(&method_names);

    // Build the original impl block with filtered methods
    let ItemImpl {
        attrs,
        defaultness,
        unsafety,
        impl_token,
        generics,
        trait_,
        self_ty,
        brace_token: _,
        items: _,
    } = input;

    let expanded = if let Some((bang, path, for_token)) = trait_ {
        quote! {
            #(#attrs)*
            #defaultness #unsafety #impl_token #generics #bang #path #for_token #self_ty {
                #get_attr_impl

                #(#filtered_items)*
            }

            #(#generated_wrappers)*
        }
    } else {
        quote! {
            #(#attrs)*
            #defaultness #unsafety #impl_token #generics #self_ty {
                #get_attr_impl

                #(#filtered_items)*
            }

            #(#generated_wrappers)*
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn rustleaf(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Item);

    match input {
        Item::Struct(item_struct) => rustleaf_struct(item_struct),
        Item::Impl(item_impl) => rustleaf_impl(item_impl),
        _ => panic!("rustleaf attribute can only be applied to structs and impl blocks"),
    }
}

fn rustleaf_struct(input: ItemStruct) -> TokenStream {
    let struct_name = &input.ident;
    let ref_name = quote::format_ident!("{}Ref", struct_name);

    // Generate the original struct unchanged
    let original_struct = quote! {
        #input
    };

    // Generate the wrapper struct with RustLeafWrapper derive
    let wrapper_struct = quote! {
        /// Generated wrapper for RustLeaf integration
        #[derive(Debug, Clone, RustLeafWrapper)]
        #[core_type(#struct_name)]
        pub struct #ref_name(std::rc::Rc<std::cell::RefCell<#struct_name>>);
    };

    // Generate trivial RustValue methods
    let rust_value_impl = generate_trivial_rust_value_impl(&ref_name, struct_name);

    // Generate get_property helper for struct fields
    let get_property_impl = generate_get_property_impl(&ref_name, &input);

    // Generate rustleaf_new constructor for the original struct
    let constructor_impl = generate_rustleaf_constructor(struct_name, &ref_name);

    let expanded = quote! {
        #original_struct

        #wrapper_struct

        #rust_value_impl

        #get_property_impl

        #constructor_impl
    };

    TokenStream::from(expanded)
}

fn generate_trivial_rust_value_impl(
    ref_name: &syn::Ident,
    struct_name: &syn::Ident,
) -> proc_macro2::TokenStream {
    let struct_name_str = struct_name.to_string();

    quote! {
        impl rustleaf::core::RustValue for #ref_name {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }

            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }

            fn dyn_clone(&self) -> Box<dyn rustleaf::core::RustValue> {
                Box::new(self.clone())
            }

            fn type_name(&self) -> Option<&str> {
                Some(#struct_name_str)
            }

            fn str(&self) -> String {
                // Simple default implementation - can be improved later
                format!("{:?}", self.borrow())
            }

            fn get_attr(&self, name: &str) -> Option<rustleaf::core::Value> {
                // Try properties first (struct fields)
                if let Some(value) = self.get_property(name) {
                    return Some(value);
                }

                // Then try methods (from impl blocks)
                self.get_method(name)
            }
        }
    }
}

fn generate_get_property_impl(
    ref_name: &syn::Ident,
    input: &ItemStruct,
) -> proc_macro2::TokenStream {
    let mut field_arms = Vec::new();

    // Analyze struct fields
    if let syn::Fields::Named(named_fields) = &input.fields {
        for field in &named_fields.named {
            if let Some(field_name) = &field.ident {
                // Only generate for public fields
                if matches!(field.vis, syn::Visibility::Public(_)) {
                    let field_name_str = field_name.to_string();
                    let field_access = field_type_to_value_conversion(
                        &field.ty,
                        quote! { self.borrow().#field_name },
                    );

                    field_arms.push(quote! {
                        #field_name_str => Some(#field_access),
                    });
                }
            }
        }
    }

    quote! {
        impl #ref_name {
            /// Generated helper for accessing struct fields
            pub fn get_property(&self, name: &str) -> Option<rustleaf::core::Value> {
                match name {
                    #(#field_arms)*
                    _ => None,
                }
            }
        }
    }
}

fn field_type_to_value_conversion(
    field_type: &syn::Type,
    field_access: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    // Handle common field types and convert to appropriate Value variants
    if let syn::Type::Path(type_path) = field_type {
        if let Some(segment) = type_path.path.segments.last() {
            match segment.ident.to_string().as_str() {
                "f64" => return quote! { rustleaf::core::Value::Float(#field_access) },
                "f32" => return quote! { rustleaf::core::Value::Float(#field_access as f64) },
                "i64" => return quote! { rustleaf::core::Value::Int(#field_access) },
                "i32" => return quote! { rustleaf::core::Value::Int(#field_access as i64) },
                "bool" => return quote! { rustleaf::core::Value::Bool(#field_access) },
                "String" => return quote! { rustleaf::core::Value::String(#field_access) },
                _ => {}
            }
        }
    }

    // Handle string slice
    if matches!(field_type, syn::Type::Reference(_)) {
        // This is a simplified check - in practice you'd want more sophisticated analysis
        return quote! { rustleaf::core::Value::String(#field_access.to_string()) };
    }

    // Default fallback - try to convert with Into<Value>
    quote! { (#field_access).into() }
}

fn rustleaf_impl(input: ItemImpl) -> TokenStream {
    // Extract the struct name from the impl block
    let struct_name = if let syn::Type::Path(type_path) = &*input.self_ty {
        &type_path.path.segments.last().unwrap().ident
    } else {
        panic!("rustleaf impl blocks must be for named types");
    };

    let ref_name = quote::format_ident!("{}Ref", struct_name);

    // Generate wrapper methods for the Ref type and track method names
    let mut wrapper_methods = Vec::new();
    let mut method_names = Vec::new();

    for item in &input.items {
        if let ImplItem::Fn(method) = item {
            // Skip constructors and private methods
            if method.sig.ident == "new" {
                continue;
            }

            let wrapper_method = generate_rustleaf_wrapper_method(method);
            wrapper_methods.push(wrapper_method);
            method_names.push(method.sig.ident.to_string());
        }
    }

    // Generate get_method helper
    let get_method_impl = generate_get_method_impl(&ref_name, &method_names);

    let expanded = quote! {
        // Keep the original impl block unchanged
        #input

        // Generate the wrapper methods for the Ref type
        impl #ref_name {
            #(#wrapper_methods)*
        }

        // Generate get_method helper
        #get_method_impl
    };

    TokenStream::from(expanded)
}

fn generate_get_method_impl(
    ref_name: &syn::Ident,
    method_names: &[String],
) -> proc_macro2::TokenStream {
    let mut method_arms = Vec::new();

    for method_name in method_names {
        let wrapper_name = quote::format_ident!("rustleaf_{}", method_name);

        method_arms.push(quote! {
            #method_name => Some(rustleaf::core::Value::from_rust(rustleaf::core::BoundMethod::new(
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

fn generate_rustleaf_wrapper_method(method: &ImplItemFn) -> proc_macro2::TokenStream {
    let method_name = &method.sig.ident;
    let wrapper_name = quote::format_ident!("rustleaf_{}", method_name);
    let method_name_str = method_name.to_string();

    // Analyze parameters (skip self)
    let mut parameters = Vec::new();
    let mut has_self_mut = false;

    for input in &method.sig.inputs {
        match input {
            syn::FnArg::Receiver(receiver) => {
                has_self_mut = receiver.mutability.is_some();
            }
            syn::FnArg::Typed(typed) => {
                if let syn::Pat::Ident(ident) = &*typed.pat {
                    parameters.push(ParameterInfo {
                        name: ident.ident.clone(),
                        type_: &typed.ty,
                    });
                }
            }
        }
    }

    // Generate wrapper based on parameters
    if parameters.is_empty() {
        generate_no_args_wrapper(&wrapper_name, method_name, &method_name_str, has_self_mut)
    } else {
        generate_args_wrapper(
            &wrapper_name,
            method_name,
            &method_name_str,
            &parameters,
            has_self_mut,
        )
    }
}

struct ParameterInfo<'a> {
    name: syn::Ident,
    type_: &'a syn::Type,
}

fn generate_no_args_wrapper(
    wrapper_name: &syn::Ident,
    method_name: &syn::Ident,
    method_name_str: &str,
    has_self_mut: bool,
) -> proc_macro2::TokenStream {
    if has_self_mut {
        quote! {
            pub fn #wrapper_name(self_value: &rustleaf::core::Value, args: rustleaf::core::Args) -> anyhow::Result<rustleaf::core::Value> {
                self_value.with_rust_value_no_args::<Self, _, _>(
                    #method_name_str,
                    stringify!(Self),
                    args,
                    |wrapper| {
                        wrapper.borrow_mut().#method_name();
                        Ok(())
                    },
                )
            }
        }
    } else {
        quote! {
            pub fn #wrapper_name(self_value: &rustleaf::core::Value, args: rustleaf::core::Args) -> anyhow::Result<rustleaf::core::Value> {
                self_value.with_rust_value_no_args::<Self, _, _>(
                    #method_name_str,
                    stringify!(Self),
                    args,
                    |wrapper| Ok(wrapper.borrow().#method_name()),
                )
            }
        }
    }
}

fn generate_args_wrapper(
    wrapper_name: &syn::Ident,
    method_name: &syn::Ident,
    method_name_str: &str,
    parameters: &[ParameterInfo],
    _has_self_mut: bool,
) -> proc_macro2::TokenStream {
    // Generate argument extraction code
    let mut arg_extractions = Vec::new();
    let mut arg_names = Vec::new();

    for (i, param) in parameters.iter().enumerate() {
        let param_name = &param.name;
        let param_name_str = param_name.to_string();
        let temp_var = quote::format_ident!("arg_{}", i);

        // For now, we'll handle common types. This can be extended.
        let extraction = if is_same_type_as_self(param.type_) {
            quote! {
                let #temp_var = args.expect(#param_name_str)?;
                let #param_name = #temp_var.expect_rust_value::<Self>(#method_name_str, stringify!(Self))?;
            }
        } else if is_f64_type(param.type_) {
            quote! {
                let #temp_var = args.expect(#param_name_str)?;
                let #param_name = #temp_var.expect_f64(#method_name_str, #param_name_str)?;
            }
        } else if is_i64_type(param.type_) {
            quote! {
                let #temp_var = args.expect(#param_name_str)?;
                let #param_name = #temp_var.expect_i64(#method_name_str, #param_name_str)?;
            }
        } else {
            // Generic fallback - try to extract as the same type
            quote! {
                let #temp_var = args.expect(#param_name_str)?;
                let #param_name = #temp_var.expect_rust_value::<Self>(#method_name_str, stringify!(Self))?;
            }
        };

        arg_extractions.push(extraction);
        arg_names.push(param_name);
    }

    // Generate method call arguments
    let method_args = if parameters.len() == 1 && is_same_type_as_self(parameters[0].type_) {
        let param_name = &parameters[0].name;
        quote! { &*#param_name.borrow() }
    } else {
        quote! { #(#arg_names),* }
    };

    quote! {
        pub fn #wrapper_name(self_value: &rustleaf::core::Value, mut args: rustleaf::core::Args) -> anyhow::Result<rustleaf::core::Value> {
            args.set_function_name(#method_name_str);

            #(#arg_extractions)*

            args.complete()?;

            let wrapper = self_value.expect_rust_value::<Self>(#method_name_str, stringify!(Self))?;
            let result = wrapper.borrow().#method_name(#method_args);
            Ok(result.into())
        }
    }
}

fn is_same_type_as_self(ty: &syn::Type) -> bool {
    // This is a simplified check - in a real implementation you'd want more sophisticated type analysis
    if let syn::Type::Reference(type_ref) = ty {
        if let syn::Type::Path(type_path) = &*type_ref.elem {
            if let Some(segment) = type_path.path.segments.last() {
                // For Vector2, we expect &Vector2 parameters to be same-type
                return segment.ident.to_string().contains("Vector2");
            }
        }
    }
    false
}

fn is_f64_type(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "f64";
        }
    }
    false
}

fn is_i64_type(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "i64";
        }
    }
    false
}

fn generate_method_wrapper(method: &ImplItemFn, is_mutating: bool) -> proc_macro2::TokenStream {
    let method_name = &method.sig.ident;
    let wrapper_name = quote::format_ident!("rustleaf_{}", method_name);
    let method_name_str = method_name.to_string();

    // Analyze the method signature to determine the wrapper pattern
    let has_other_params = method.sig.inputs.len() > 1;

    // Get the return type
    let return_type = match &method.sig.output {
        ReturnType::Default => quote! { () },
        ReturnType::Type(_, ty) => quote! { #ty },
    };

    if !has_other_params {
        // No-argument method
        if is_mutating {
            quote! {
                pub fn #wrapper_name(self_value: &rustleaf::core::Value, args: rustleaf::core::Args) -> anyhow::Result<rustleaf::core::Value> {
                    self_value.with_rust_value_no_args::<Self, #return_type, _>(
                        #method_name_str,
                        stringify!(Self),
                        args,
                        |rust_val| {
                            let mut borrowed = rust_val.borrow_mut();
                            Ok(borrowed.#method_name())
                        },
                    )
                }
            }
        } else {
            quote! {
                pub fn #wrapper_name(self_value: &rustleaf::core::Value, args: rustleaf::core::Args) -> anyhow::Result<rustleaf::core::Value> {
                    self_value.with_rust_value_no_args::<Self, #return_type, _>(
                        #method_name_str,
                        stringify!(Self),
                        args,
                        |rust_val| Ok(rust_val.borrow().#method_name()),
                    )
                }
            }
        }
    } else {
        // Single-argument method (assuming same type for now)
        quote! {
            pub fn #wrapper_name(self_value: &rustleaf::core::Value, args: rustleaf::core::Args) -> anyhow::Result<rustleaf::core::Value> {
                self_value.with_rust_value_same_type::<Self, #return_type, _>(
                    #method_name_str,
                    stringify!(Self),
                    "other",
                    args,
                    |rust_val, other_val| {
                        Ok(rust_val.borrow().#method_name(&*other_val.borrow()))
                    },
                )
            }
        }
    }
}

fn generate_get_attr_impl(method_names: &[String]) -> proc_macro2::TokenStream {
    let match_arms = method_names.iter().map(|name| {
        let wrapper_name = quote::format_ident!("rustleaf_{}", name);
        quote! {
            #name => {
                Some(rustleaf::core::Value::from_rust(rustleaf::core::BoundMethod::new(
                    &rustleaf::core::Value::rust_value(self.dyn_clone()),
                    Self::#wrapper_name,
                )))
            }
        }
    });

    quote! {
        fn get_attr(&self, name: &str) -> Option<rustleaf::core::Value> {
            match name {
                #(#match_arms)*
                _ => None,
            }
        }
    }
}

fn generate_rustleaf_constructor(
    struct_name: &syn::Ident,
    ref_name: &syn::Ident,
) -> proc_macro2::TokenStream {
    quote! {
        impl #struct_name {
            /// Generated constructor function for RustLeaf integration
            pub fn rustleaf_new(mut args: rustleaf::core::Args) -> anyhow::Result<rustleaf::core::Value> {
                args.set_function_name("new");

                // Extract constructor arguments - for Vector2, we expect (x: f64, y: f64)
                let x = args.expect("x")?.expect_f64("new", "x")?;
                let y = args.expect("y")?.expect_f64("new", "y")?;

                args.complete()?;

                // Create the struct instance
                let instance = Self::new(x, y);

                // Wrap it in the Ref type and return as Value
                let wrapper = #ref_name::new(instance);
                Ok(rustleaf::core::Value::rust_value(Box::new(wrapper)))
            }
        }
    }
}
