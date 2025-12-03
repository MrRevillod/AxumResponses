use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Error, Fields, Ident, Variant, spanned::Spanned};

use crate::{ErrorValue, VariantConfig};

pub fn generate_match_arm(
    enum_name: &Ident,
    variant: &Variant,
    config: &VariantConfig,
) -> syn::Result<TokenStream> {
    let variant_name = &variant.ident;
    let status_code = config.code.as_ref().unwrap().as_u16();

    let pattern = generate_pattern(enum_name, variant_name, &variant.fields)?;
    let builder = generate_builder(status_code, config);

    Ok(quote! {
        #pattern => { #builder }
    })
}

fn generate_pattern(
    enum_name: &Ident,
    variant_name: &Ident,
    fields: &Fields,
) -> syn::Result<TokenStream> {
    match fields {
        Fields::Unit => Ok(quote! { #enum_name::#variant_name }),

        Fields::Unnamed(f) => Err(Error::new(
            f.span(),
            "tuple variants are not supported yet, use named fields instead",
        )),

        Fields::Named(fields) => {
            let names: Vec<_> = fields
                .named
                .iter()
                .filter_map(|f| f.ident.as_ref())
                .collect();

            Ok(quote! { #enum_name::#variant_name { #(#names),* } })
        }
    }
}

fn generate_builder(status_code: u16, config: &VariantConfig) -> TokenStream {
    let mut builder = quote! {
        ::axum_responses::JsonResponse::builder_u16(#status_code)
    };

    if let Some(message) = &config.message {
        builder = quote! { #builder.message(#message) };
    }

    if let Some(error) = &config.error {
        let value = error_value_to_tokens(error);
        builder = quote! { #builder.error(#value) };
    }

    if let Some(errors) = &config.errors {
        let value = error_value_to_tokens(errors);
        builder = quote! { #builder.errors(#value) };
    }

    builder
}

fn error_value_to_tokens(error: &ErrorValue) -> TokenStream {
    match error {
        ErrorValue::Literal(text) => quote! { #text },

        ErrorValue::FieldName(name) => {
            let ident = format_ident!("{}", name);
            quote! { #ident }
        }

        ErrorValue::FormatString(template) => quote! { format!(#template) },
    }
}

pub fn generate_function_enum_variants(variant: &Variant) -> TokenStream {
    let variant_name = &variant.ident;

    match &variant.fields {
        Fields::Unit => {
            quote! {}
        }

        Fields::Unnamed(_) => {
            quote! {}
        }

        Fields::Named(fields) => {
            let param_names: Vec<_> = fields
                .named
                .iter()
                .filter_map(|f| f.ident.as_ref())
                .collect();

            let param_defs: Vec<_> = fields
                .named
                .iter()
                .map(|f| {
                    let name = &f.ident;
                    let ty = &f.ty;
                    quote! { #name: #ty }
                })
                .collect();

            quote! {
                #[allow(non_snake_case)]
                pub fn #variant_name(#(#param_defs),*) -> Self {
                    Self::#variant_name { #(#param_names),* }
                }
            }
        }
    }
}
