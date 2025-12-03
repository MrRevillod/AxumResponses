use axum::http::StatusCode;
use syn::{Attribute, Error, Expr, ExprLit, ExprPath, Ident, Lit, spanned::Spanned};

#[derive(Default)]
pub struct VariantConfig {
    pub code: Option<StatusCode>,
    pub message: Option<String>,
    pub error: Option<ErrorValue>,
    pub errors: Option<ErrorValue>,
}

/// #[error("text")] | #[error(field)] | #[error("Value is {field}")]
pub enum ErrorValue {
    Literal(String),
    FieldName(String),
    FormatString(String),
}

pub fn parse_variant_config(ident: &Ident, attrs: &[Attribute]) -> syn::Result<VariantConfig> {
    let mut config = VariantConfig::default();

    for attr in attrs {
        let Some(name) = attr.path().get_ident() else {
            continue;
        };

        match name.to_string().as_str() {
            "code" => config.code = Some(parse_u16_to_status_code(attr)?),
            "message" => config.message = Some(get_literal_string(attr)?),
            "error" => config.error = Some(parse_error_value(attr)?),
            "errors" => config.errors = Some(parse_error_value(attr)?),
            _ => {}
        }
    }

    if config.code.is_none() {
        return Err(Error::new_spanned(ident, "Missing `code` attribute"));
    }

    Ok(config)
}

fn get_literal_string(attr: &Attribute) -> syn::Result<String> {
    let lit = match attr.parse_args::<Expr>()? {
        Expr::Lit(e) => match e.lit {
            Lit::Str(s) => Ok(s),
            other => Err(Error::new(other.span(), "expected string literal")),
        },
        other => Err(Error::new(other.span(), "expected string literal")),
    };

    Ok(lit?.value())
}

fn parse_u16_to_status_code(attr: &Attribute) -> syn::Result<StatusCode> {
    let lit = match attr.parse_args::<Expr>()? {
        Expr::Lit(e) => match e.lit {
            Lit::Int(i) => Ok(i),
            other => Err(Error::new(other.span(), "expected integer literal")),
        },

        other => Err(Error::new(other.span(), "expected integer literal")),
    }?;

    let code = lit.base10_parse::<u16>()?;

    StatusCode::from_u16(code).map_err(|_| Error::new(lit.span(), "invalid HTTP status code"))
}

fn parse_error_value(attr: &Attribute) -> syn::Result<ErrorValue> {
    match attr.parse_args::<Expr>()? {
        Expr::Lit(e) => parse_literal_error_value(e),
        Expr::Path(p) => parse_field_error_value(p),
        other => Err(Error::new(
            other.span(),
            "expected string literal or identifier",
        )),
    }
}

fn parse_literal_error_value(expr: ExprLit) -> syn::Result<ErrorValue> {
    match expr.lit {
        Lit::Str(s) => {
            let value = s.value();
            if value.contains('{') {
                Ok(ErrorValue::FormatString(value))
            } else {
                Ok(ErrorValue::Literal(value))
            }
        }
        other => Err(Error::new(other.span(), "expected string literal")),
    }
}

fn parse_field_error_value(expr: ExprPath) -> syn::Result<ErrorValue> {
    expr.path
        .get_ident()
        .map(|id| ErrorValue::FieldName(id.to_string()))
        .ok_or_else(|| Error::new(expr.span(), "expected simple identifier"))
}
