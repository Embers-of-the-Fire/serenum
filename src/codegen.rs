use convert_case::{Case, Casing};
use darling::{FromDeriveInput, FromVariant};
use itertools::MultiUnzip;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

#[derive(FromDeriveInput)]
#[darling(attributes(strenum), supports(enum_unit))]
pub struct Config {
    ident: syn::Ident,
    data: darling::ast::Data<VariantConfig, ()>,
    #[darling(default)]
    from: Option<String>,
    #[darling(default)]
    to: Option<String>,
}

#[derive(FromVariant)]
#[darling(attributes(strenum))]
pub struct VariantConfig {
    ident: syn::Ident,
    text: String,
    #[darling(default)]
    const_name: Option<String>,
}

pub fn codegen(cfg: &Config) -> Result<TokenStream, String> {
    let variants = cfg
        .data
        .as_ref()
        .take_enum()
        .ok_or("target must be an enum".to_owned())?;
    let (ident, const_name, text): (Vec<_>, Vec<_>, Vec<_>) = variants
        .iter()
        .map(|v| {
            let const_name = v
                .const_name
                .clone()
                .unwrap_or_else(|| v.ident.to_string().to_case(Case::UpperSnake));
            (&v.ident, format_ident!("{}", const_name), &v.text)
        })
        .multiunzip();
    let from_fn_name = cfg.from.as_ref().map(AsRef::as_ref).unwrap_or("from_text");
    let to_fn_name = cfg.to.as_ref().map(AsRef::as_ref).unwrap_or("text");
    let from_fn_ident = format_ident!("{}", from_fn_name);
    let to_fn_ident = format_ident!("{}", to_fn_name);
    let enum_name = &cfg.ident;

    let code = quote! {
        impl #enum_name {
            #(pub const #const_name: &'static str = #text;)*

            pub fn #from_fn_ident(text: &impl ::core::convert::AsRef<str>) -> ::core::option::Option<Self> {
                match <_ as ::core::convert::AsRef<str>>::as_ref(text) {
                    #(Self::#const_name => ::core::option::Option::Some(Self::#ident),)*
                    _ => ::core::option::Option::None
                }
            }

            pub const fn #to_fn_ident(&self) -> &str {
                match self {
                    #(Self::#ident => Self::#const_name,)*
                }
            }
        }
    };

    #[cfg(not(feature = "serde"))]
    return Ok(code);

    #[cfg(feature = "serde")]
    return {
        let inner_impl_name = format_ident!("__impl_{}", enum_name);
        let serde_impl = quote! {
            impl ::serde::Serialize for #enum_name {
                fn serialize<S>(&self, serializer: S) -> ::core::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::Serializer,
                {
                    serializer.serialize_str(self.#to_fn_ident())
                }
            }

            impl<'de> ::serde::Deserialize<'de> for #enum_name {
                fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, D::Error>
                where
                    D: ::serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[derive(::serde::Deserialize)]
                    enum #inner_impl_name {
                        #(
                            #[serde(rename = #text)]
                            #ident,
                        )*
                    }

                    let result = <#inner_impl_name as ::serde::Deserialize<'de>>::deserialize(deserializer)?;
                    Ok(match result {
                        #(
                            #inner_impl_name::#ident => #enum_name::#ident,
                        )*
                    })
                }
            }
        };

        Ok(quote! {
            #serde_impl
            #code
        })
    };
}
