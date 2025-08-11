use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Item, ItemImpl, ItemStruct};

mod function_wrapper;
mod method_wrapper;

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
pub fn rustleaf(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Item);

    match input {
        Item::Struct(item_struct) => rustleaf_struct(item_struct),
        Item::Impl(item_impl) => method_wrapper::rustleaf_impl(item_impl),
        Item::Fn(item_fn) => function_wrapper::rustleaf_fn(item_fn),
        _ => {
            panic!("rustleaf attribute can only be applied to structs, impl blocks, and functions")
        }
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

    // Generate BorrowValueAs implementations for this user type
    let borrow_value_impl = generate_borrow_value_impl(struct_name, &ref_name);

    let expanded = quote! {
        #original_struct

        #wrapper_struct

        #rust_value_impl

        #get_property_impl

        #borrow_value_impl
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
                "i16" => return quote! { rustleaf::core::Value::Int(#field_access as i64) },
                "i8" => return quote! { rustleaf::core::Value::Int(#field_access as i64) },
                "u64" => return quote! { rustleaf::core::Value::Int(#field_access as i64) },
                "u32" => return quote! { rustleaf::core::Value::Int(#field_access as i64) },
                "u16" => return quote! { rustleaf::core::Value::Int(#field_access as i64) },
                "u8" => return quote! { rustleaf::core::Value::Int(#field_access as i64) },
                "usize" => return quote! { rustleaf::core::Value::Int(#field_access as i64) },
                "isize" => return quote! { rustleaf::core::Value::Int(#field_access as i64) },
                "bool" => return quote! { rustleaf::core::Value::Bool(#field_access) },
                "String" => return quote! { rustleaf::core::Value::String(#field_access.clone()) },
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

fn generate_borrow_value_impl(
    struct_name: &syn::Ident,
    ref_name: &syn::Ident,
) -> proc_macro2::TokenStream {
    quote! {
        // Generate BorrowValueAs implementation for this user-defined type
        impl rustleaf::core::BorrowValueAs<#struct_name> for rustleaf::core::Value {
            type Guard<'a> = std::cell::Ref<'a, #struct_name>;

            fn borrow_value_as(&self) -> Result<Self::Guard<'_>, anyhow::Error> {
                if let Some(rust_value) = self.downcast_rust_value::<#ref_name>() {
                    Ok(rust_value.borrow())
                } else {
                    Err(anyhow::anyhow!("Cannot borrow {} as {}", self.type_name(), stringify!(#struct_name)))
                }
            }
        }

        // Generate BorrowMutValueAs implementation for this user-defined type
        impl rustleaf::core::BorrowMutValueAs<#struct_name> for rustleaf::core::Value {
            type Guard<'a> = std::cell::RefMut<'a, #struct_name>;

            fn borrow_mut_value_as(&mut self) -> Result<Self::Guard<'_>, anyhow::Error> {
                if let Some(rust_value) = self.downcast_rust_value::<#ref_name>() {
                    Ok(rust_value.borrow_mut())
                } else {
                    Err(anyhow::anyhow!("Cannot borrow {} as mut {}", self.type_name(), stringify!(#struct_name)))
                }
            }
        }
    }
}
