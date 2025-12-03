mod parse;
use parse::{ErrorValue, VariantConfig, parse_variant_config};

mod generation;
use generation::generate_match_arm;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Error, parse_macro_input};

use crate::generation::generate_function_enum_variants;

#[proc_macro_derive(HttpError, attributes(code, error, errors, message))]
pub fn derive_error_macro(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    let Data::Enum(enum_data) = &input.data else {
        return Error::new_spanned(input.ident, "`Error` macro can only be derived for enums")
            .to_compile_error()
            .into();
    };

    let self_ty = &input.ident;
    let mut match_arms = Vec::new();
    let mut variant_functions = Vec::new();

    for variant in &enum_data.variants {
        let variant_config = match parse_variant_config(&variant.ident, &variant.attrs) {
            Ok(cfg) => cfg,
            Err(e) => return e.to_compile_error().into(),
        };

        let generated = match generate_match_arm(self_ty, variant, &variant_config) {
            Ok(arm) => arm,
            Err(e) => return e.to_compile_error().into(),
        };

        let generated_variant_function = generate_function_enum_variants(variant);

        match_arms.push(generated);
        variant_functions.push(generated_variant_function);
    }

    quote! {
        impl From<#self_ty> for ::axum_responses::JsonResponse {
            fn from(err: #self_ty) -> Self {
                match err {
                    #(#match_arms)*
                }
            }
        }

        impl ::std::error::Error for #self_ty {}

        impl std::fmt::Display for #self_ty {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:?}", self)
            }
        }

        impl #self_ty {
            #(#variant_functions)*
        }
    }
    .into()
}
