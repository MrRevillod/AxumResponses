
use std::collections::HashMap;

use serde::Serialize;
use axum::http::StatusCode;
use serde_json::{to_value, Value};

pub trait ToJson where Self: Serialize {

    /// Convert the struct to a JSON value or a Null value
    /// The provided struct must implement the Serialize trait
    /// 
    /// # Example
    /// 
    /// ```
    /// use serde::Serialize;
    /// use serde_json::Value;
    /// use axum_responses::extra::ToJson;
    /// 
    /// #[derive(Serialize)]
    /// struct MyStruct {
    ///    name: String,
    ///    age: u8
    /// }
    /// 
    /// impl ToJson for MyStruct {}
    /// 
    /// let my_struct = MyStruct {
    ///    name: "John".to_string(),
    ///    age: 25
    /// };
    /// 
    /// let json_value: Value = my_struct.to_json();

    fn to_json(&self) -> Value {
        to_value(self).unwrap_or(Value::Null)
    }
}

impl ToJson for HashMap<String, String> {}
impl ToJson for HashMap<&'static str, &'static str> {}
impl<T> ToJson for Vec<T> where T: Serialize {}

/// Convert a u16 status code to a StatusCode
/// 
/// If the provided code is not a valid status code,
/// the function will return a 500 Internal Server Error

pub fn to_http_status(code: u16) -> StatusCode {
    StatusCode::from_u16(code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn res_type(code: &StatusCode) -> &str {

    match code.is_success() {
        true => "success",
        false => "error"
    }
}
