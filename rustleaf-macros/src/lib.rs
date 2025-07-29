use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, ImplItem, ImplItemFn, ItemImpl, ReturnType};

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
