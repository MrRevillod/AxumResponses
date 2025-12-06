use axum::http::StatusCode;
use syn::{Attribute, Error, Ident, spanned::Spanned};

#[derive(Debug, Clone)]
pub enum MessageValue {
    Static(String),
    Field(String),
}

#[derive(Default)]
pub struct HttpErrorConfig {
    /// Delegate to inner type's `From<T> for Json`
    pub transparent: bool,
    /// HTTP status code
    pub code: Option<StatusCode>,
    /// Custom message (static string or field reference)
    pub message: Option<MessageValue>,
    /// Named field to include as "error" in response
    pub error_field: Option<String>,
    /// Named field to include as "errors" in response
    pub errors_field: Option<String>,
}

impl HttpErrorConfig {
    pub fn from_attrs(ident: &Ident, attrs: &[Attribute]) -> syn::Result<Self> {
        let mut config = Self::default();

        for attr in attrs.iter().filter(|a| a.path().is_ident("http")) {
            config.parse_http_attr(attr)?;
        }

        config.validate(ident)?;
        Ok(config)
    }

    fn parse_http_attr(&mut self, attr: &Attribute) -> syn::Result<()> {
        attr.parse_nested_meta(|meta| {
            let ident = meta.path.get_ident().ok_or_else(|| {
                Error::new(meta.path.span(), "expected identifier")
            })?;

            match ident.to_string().as_str() {
                "transparent" => {
                    self.transparent = true;
                }
                "code" => {
                    let lit: syn::LitInt = meta.value()?.parse()?;
                    let code = lit.base10_parse::<u16>()?;
                    self.code = Some(StatusCode::from_u16(code).map_err(|_| {
                        Error::new(lit.span(), "invalid HTTP status code")
                    })?);
                }
                "message" => {
                    if meta.input.peek(syn::Token![=]) {
                        meta.input.parse::<syn::Token![=]>()?;
                        if let Ok(lit) = meta.input.parse::<syn::LitStr>() {
                            self.message = Some(MessageValue::Static(lit.value()));
                        } else {
                            let field: Ident = meta.input.parse()?;
                            self.message =
                                Some(MessageValue::Field(field.to_string()));
                        }
                    } else {
                        let field: Ident = meta.input.parse()?;
                        self.message = Some(MessageValue::Field(field.to_string()));
                    }
                }
                "error" => {
                    let field: Ident = meta.value()?.parse()?;
                    self.error_field = Some(field.to_string());
                }
                "errors" => {
                    let field: Ident = meta.value()?.parse()?;
                    self.errors_field = Some(field.to_string());
                }
                other => {
                    return Err(Error::new(
                        ident.span(),
                        format!("unknown attribute `{other}`"),
                    ));
                }
            }
            Ok(())
        })
    }

    fn validate(&self, ident: &Ident) -> syn::Result<()> {
        if !self.transparent && self.code.is_none() {
            return Err(Error::new_spanned(
                ident,
                "missing `transparent` or `code` in #[http(...)]",
            ));
        }

        if self.transparent && self.code.is_some() {
            return Err(Error::new_spanned(
                ident,
                "cannot use both `transparent` and `code`",
            ));
        }

        if self.transparent {
            if self.message.is_some() {
                return Err(Error::new_spanned(
                    ident,
                    "`message` is not valid with `transparent`",
                ));
            }

            if self.error_field.is_some() || self.errors_field.is_some() {
                return Err(Error::new_spanned(
                    ident,
                    "`error`/`errors` is not valid with `transparent`",
                ));
            }
        }

        Ok(())
    }

    pub fn message(&self) -> Option<MessageValue> {
        self.message.clone()
    }

    pub fn default_message(&self) -> String {
        self.code
            .as_ref()
            .map(|c| c.canonical_reason().unwrap_or("Unknown Error"))
            .unwrap_or("Unknown Error")
            .to_string()
    }
}
