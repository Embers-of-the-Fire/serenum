use codegen::Config;
use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

mod codegen;

#[proc_macro_derive(StrEnum, attributes(strenum))]
pub fn strenum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let cfg = match Config::from_derive_input(&input) {
        Ok(cfg) => cfg,
        Err(e) => {
            return e.write_errors().into();
        }
    };
    let code = codegen::codegen(&cfg).unwrap_or_else(|t| quote! { ::core::compile_error!(#t); });
    code.into()
}
