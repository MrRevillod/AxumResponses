
use serde_json::{json, Value};
use crate::{res_type, to_http_status};

use axum::{
    
    Json, response::{
        IntoResponse, 
        Response as HttpResponse
    }, 
};

/// `AxumResponse` data type that represents an HTTP response.
 
pub type AxumResponse = Result<Response, Response>;

/// `AxumResult` data type that represents a response 
/// from some service in the API.
/// 
/// ### Parameters
/// 
/// * `T`: Data type of the response.
/// 
/// ### Example
/// Returns a type T if the response is successful, 
/// otherwise it returns a negative `ApiResponse`, 
/// that is, an error HttpResponse.
 
pub type AxumResult<T> = Result<T, Response>;

pub enum Response {
    
    /// `Standard` is a standard response.
    /// 
    /// ### Parameters
    /// 
    /// * `u16`: HTTP status code.
    /// * `&'static str`: Response message.
     
    Standard(u16, &'static str),

    /// `JsonData` is a response that contains data.
    /// 
    /// ### Parameters
    /// 
    /// * `u16`: HTTP status code.
    /// * `&'static str`: Response message.
    /// * `&'static str`: Name | key of the Value.
    /// * `Value`: The data of the response.
    /// 
    /// ### Example
    /// 
    /// ```rust
    /// use axum_responses::http::Response;
    /// use axum_responses::ToJson;
    /// 
    /// use serde::{Serialize, Deserialize};
    /// use serde_json::Value;
    /// 
    /// #[derive(Serialize, Deserialize)]
    /// struct TestStruct {
    ///    field: String
    /// }
    /// 
    /// impl ToJson for TestStruct {}
    /// 
    /// let test_struct = TestStruct {
    ///     field: "value".to_string()
    /// };
    /// 
    /// let response = Response::JsonData(
    ///     200, "Success", "data", test_struct.to_json()
    /// );
    /// ```

    JsonData(u16, &'static str, &'static str, Value)
}

impl IntoResponse for Response {

    fn into_response(self) -> HttpResponse {
        
        match self {

            Response::Standard(status, message) => {

                let code = to_http_status(status);

                let data = json!({ 
                    "status_code": code.as_u16(), 
                    "message": message, 
                    "type": res_type(&code)
                });

                return (code, Json(data)).into_response()
            },

            Response::JsonData(status, message, data_name, data) => {

                let code = to_http_status(status);

                let data = json!({
                    "status_code": code.as_u16(), 
                    "message": message, 
                    data_name: data,
                    "type": res_type(&code)
                });

                return (code, Json(data)).into_response()
            }
        }
    }
}
