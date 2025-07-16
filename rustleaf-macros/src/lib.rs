use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream}, 
    parse_macro_input, 
    punctuated::Punctuated,
    token::{Comma, FatArrow},
    Ident, Token
};

struct BinaryOpLevel {
    method_name: Ident,
    next_method: Ident,
    operators: Punctuated<OpMapping, Comma>,
}

struct OpMapping {
    token_type: Ident,
    binary_op: Ident,
}

impl Parse for OpMapping {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let token_type = input.parse()?;
        if input.peek(FatArrow) {
            input.parse::<FatArrow>()?;
            let binary_op = input.parse()?;
            Ok(OpMapping { token_type, binary_op })
        } else {
            let binary_op = token_type.clone();
            Ok(OpMapping { token_type, binary_op })
        }
    }
}

impl Parse for BinaryOpLevel {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let method_name = input.parse()?;
        input.parse::<Token![->]>()?;
        let next_method = input.parse()?;
        input.parse::<Token![:]>()?;
        let content;
        syn::bracketed!(content in input);
        let operators = content.parse_terminated(OpMapping::parse, Comma)?;
        
        Ok(BinaryOpLevel {
            method_name,
            next_method,
            operators,
        })
    }
}

struct BinaryOpsInput {
    levels: Punctuated<BinaryOpLevel, Comma>,
}

impl Parse for BinaryOpsInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let levels = input.parse_terminated(BinaryOpLevel::parse, Comma)?;
        Ok(BinaryOpsInput { levels })
    }
}

#[proc_macro_attribute]
pub fn binary_ops(args: TokenStream, input: TokenStream) -> TokenStream {
    let input_ast = parse_macro_input!(input as syn::ItemImpl);
    let ops_input = parse_macro_input!(args as BinaryOpsInput);
    
    let mut methods = Vec::new();
    
    for level in ops_input.levels {
        let method_name = &level.method_name;
        let next_method = &level.next_method;
        
        let match_arms = level.operators.iter().map(|op| {
            let token_type = &op.token_type;
            let binary_op = &op.binary_op;
            quote! {
                TokenType::#token_type => {
                    self.advance();
                    BinaryOperator::#binary_op
                }
            }
        });
        
        let method = quote! {
            pub fn #method_name(&mut self) -> Option<AstNode> {
                let mut expr = self.#next_method()?;
                
                loop {
                    let op = match &self.peek().token_type {
                        #(#match_arms)*
                        _ => break,
                    };
                    
                    let location = self.current_location();
                    let right = self.#next_method()?;
                    expr = AstNode::BinaryOp {
                        left: Box::new(expr),
                        operator: op,
                        right: Box::new(right),
                        location,
                    };
                }
                
                Some(expr)
            }
        };
        
        methods.push(method);
    }
    
    let self_ty = &input_ast.self_ty;
    let existing_items = &input_ast.items;
    
    let expanded = quote! {
        impl #self_ty {
            #(#existing_items)*
            #(#methods)*
        }
    };
    
    TokenStream::from(expanded)
}