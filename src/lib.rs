//! # `strenum`
//! 
//! ## Example usage
//! 
//! Source code:
//! 
//! ```rust
//! #[derive(StrEnum)]
//! pub enum Order {
//!     #[strenum(text = "full")]
//!     Full,
//!     #[strenum(text = "short")]
//!     Short,
//! }
//! assert_eq!(Order::Full.text(), "full");
//! assert_eq!(Order::Short.text(), "short");
//! ```
//! 
//! Expanded:
//! 
//! ```rust
//! impl ::serde::Serialize for Order {
//!     fn serialize<S>(&self, serializer: S) -> ::core::result::Result<S::Ok, S::Error>
//!     where
//!         S: ::serde::Serializer,
//!     {
//!         serializer.serialize_str(self.text())
//!     }
//! }
//! impl<'de> ::serde::Deserialize<'de> for Order {
//!     fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, D::Error>
//!     where
//!         D: ::serde::Deserializer<'de>,
//!     {
//!         #[allow(non_camel_case_types)]
//!         #[derive(::serde::Deserialize)]
//!         enum __impl_Order {
//!             #[serde(rename = "full")]
//!             Full,
//!             #[serde(rename = "short")]
//!             Short,
//!         }
//!         let result = <__impl_Order as ::serde::Deserialize<'de>>::deserialize(deserializer)?;
//!         Ok(match result {
//!             __impl_Order::Full => Order::Full,
//!             __impl_Order::Short => Order::Short,
//!         })
//!     }
//! }
//! impl Order {
//!     pub const FULL: &'static str = "full";
//!     pub const SHORT: &'static str = "short";
//!     pub fn from_text(text: &impl ::core::convert::AsRef<str>) -> ::core::option::Option<Self> {
//!         match <_ as ::core::convert::AsRef<str>>::as_ref(text) {
//!             Self::FULL => ::core::option::Option::Some(Self::Full),
//!             Self::SHORT => ::core::option::Option::Some(Self::Short),
//!             _ => ::core::option::Option::None,
//!         }
//!     }
//!     pub const fn text(&self) -> &str {
//!         match self {
//!             Self::Full => Self::FULL,
//!             Self::Short => Self::SHORT,
//!         }
//!     }
//! }
//! ```
//! 
//! ## Features
//! 
//! | name    | default or not | description                                                          |
//! | ------- | -------------- | -------------------------------------------------------------------- |
//! | `serde` | default        | Generate `serde::Serialize` and `serde::Deserialize` implementation. |
//! 
//! ## Macro configuration
//! 
//! ### Enum level
//! 
//! | name   | syntax                        | required or not      | description                         |
//! | ------ | ----------------------------- | -------------------- | ----------------------------------- |
//! | `from` | `from = "some_function_name"` | default: `from_text` | The generated string parser's name. |
//! | `to`   | `to = "some_function_name"`   | default: `text`      | The generated text getter's name.   |
//! 
//! ### Variant level
//! 
//! | name         | syntax                      | required or not                                   | description                                      |
//! | ------------ | --------------------------- | ------------------------------------------------- | ------------------------------------------------ |
//! | `text`       | `text = "variant_repr"`     | required                                          | You string representation of the variant.        |
//! | `const_name` | `const_name = "CONST_NAME"` | default: UPPER_SNAKE_CASE of your variant's name. | The associated const item's name of the variant. |

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
