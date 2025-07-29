use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, ItemImpl};

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
