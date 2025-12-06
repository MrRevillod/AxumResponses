mod parse;

use parse::HttpErrorConfig;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Data, DeriveInput, Fields, Ident};

pub fn derive(input: DeriveInput) -> syn::Result<TokenStream> {
    let enum_name = &input.ident;

    let variants = match &input.data {
        Data::Enum(data) => &data.variants,
        _ => {
            return Err(syn::Error::new_spanned(
                input,
                "HttpError can only be derived for enums",
            ));
        }
    };

    let mut from_arms = Vec::new();

    for variant in variants {
        let config = HttpErrorConfig::from_attrs(&variant.ident, &variant.attrs)?;

        if config.transparent {
            match &variant.fields {
                Fields::Unnamed(f) if f.unnamed.len() == 1 => {}
                _ => {
                    return Err(syn::Error::new_spanned(
                        &variant.ident,
                        "transparent variants must have exactly one unnamed field",
                    ));
                }
            }
        }

        if (config.error_field.is_some() || config.errors_field.is_some())
            && !matches!(&variant.fields, Fields::Named(_))
        {
            return Err(syn::Error::new_spanned(
                &variant.ident,
                "`error` and `errors` can only be used with named fields",
            ));
        }

        from_arms.push(generate_from_arm(
            enum_name,
            &variant.ident,
            &variant.fields,
            &config,
        ));
    }

    Ok(quote! {
        impl From<#enum_name> for ::axum_responses::JsonResponse {
            fn from(err: #enum_name) -> Self {
                match err {
                    #(#from_arms)*
                }
            }
        }

        impl axum::response::IntoResponse for #enum_name {
            fn into_response(self) -> axum::response::Response {
                ::axum_responses::JsonResponse::from(self).into_response()
            }
        }
    })
}

fn generate_from_arm(
    enum_name: &Ident,
    variant_name: &Ident,
    fields: &Fields,
    config: &HttpErrorConfig,
) -> TokenStream {
    if !config.transparent {
        let pattern = generate_pattern(enum_name, variant_name, fields);
        let builder = generate_json_builder(fields, config);

        return quote! { #pattern => #builder, };
    }

    quote! {
        #enum_name::#variant_name(inner) => ::axum_responses::JsonResponse::from(inner),
    }
}

fn generate_pattern(
    enum_name: &Ident,
    variant_name: &Ident,
    fields: &Fields,
) -> TokenStream {
    match fields {
        Fields::Named(named) => {
            let field_names: Vec<_> = named.named.iter().map(|f| &f.ident).collect();
            quote! { #enum_name::#variant_name { #(#field_names),* } }
        }
        Fields::Unnamed(_) => {
            quote! { #enum_name::#variant_name(_inner) }
        }
        Fields::Unit => {
            quote! { #enum_name::#variant_name }
        }
    }
}

fn generate_json_builder(fields: &Fields, config: &HttpErrorConfig) -> TokenStream {
    let code = config.code.as_ref().unwrap().as_u16();
    let message = config.message();

    let base = quote! {
        ::axum_responses::JsonResponse::status(#code).message(#message)
    };

    match fields {
        Fields::Named(_) => {
            if let Some(field) = &config.error_field {
                let field_ident = Ident::new(field, Span::call_site());
                quote! { #base.error(#field_ident) }
            } else if let Some(field) = &config.errors_field {
                let field_ident = Ident::new(field, Span::call_site());
                quote! { #base.errors(#field_ident) }
            } else {
                base
            }
        }
        _ => base,
    }
}
