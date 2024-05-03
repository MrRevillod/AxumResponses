
use serde_json::json;

use crate::{
    res_type, 
    Response, 
    HttpResponse, 
    to_http_status,
};

use axum::{
    
    Json, response::{
        IntoResponse, 
        Response as AxumResponse,
    }, 
};

impl IntoResponse for Response {

    fn into_response(self) -> AxumResponse {
        
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

impl IntoResponse for HttpResponse {
    fn into_response(self) -> AxumResponse {
        match self {
            HttpResponse::CONTINUE => Response::Standard(100, "Continue").into_response(),
            HttpResponse::SWITCHING_PROTOCOLS => Response::Standard(101, "Switching Protocols").into_response(),
            HttpResponse::OK => Response::Standard(200, "OK").into_response(),
            HttpResponse::CREATED => Response::Standard(201, "Created").into_response(),
            HttpResponse::ACCEPTED => Response::Standard(202, "Accepted").into_response(),
            HttpResponse::NON_AUTHORITATIVE_INFORMATION => Response::Standard(203, "Non-Authoritative Information").into_response(),
            HttpResponse::NO_CONTENT => Response::Standard(204, "No Content").into_response(),
            HttpResponse::RESET_CONTENT => Response::Standard(205, "Reset Content").into_response(),
            HttpResponse::PARTIAL_CONTENT => Response::Standard(206, "Partial Content").into_response(),
            HttpResponse::MULTIPLE_CHOICES => Response::Standard(300, "Multiple Choices").into_response(),
            HttpResponse::MOVED_PERMANENTLY => Response::Standard(301, "Moved Permanently").into_response(),
            HttpResponse::FOUND => Response::Standard(302, "Found").into_response(),
            HttpResponse::SEE_OTHER => Response::Standard(303, "See Other").into_response(),
            HttpResponse::NOT_MODIFIED => Response::Standard(304, "Not Modified").into_response(),
            HttpResponse::USE_PROXY => Response::Standard(305, "Use Proxy").into_response(),
            HttpResponse::TEMPORARY_REDIRECT => Response::Standard(307, "Temporary Redirect").into_response(),
            HttpResponse::BAD_REQUEST => Response::Standard(400, "Bad Request").into_response(),
            HttpResponse::UNAUTHORIZED => Response::Standard(401, "Unauthorized").into_response(),
            HttpResponse::PAYMENT_REQUIRED => Response::Standard(402, "Payment Required").into_response(),
            HttpResponse::FORBIDDEN => Response::Standard(403, "Forbidden").into_response(),
            HttpResponse::NOT_FOUND => Response::Standard(404, "Not Found").into_response(),
            HttpResponse::METHOD_NOT_ALLOWED => Response::Standard(405, "Method Not Allowed").into_response(),
            HttpResponse::NOT_ACCEPTABLE => Response::Standard(406, "Not Acceptable").into_response(),
            HttpResponse::PROXY_AUTHENTICATION_REQUIRED => Response::Standard(407, "Proxy Authentication Required").into_response(),
            HttpResponse::REQUEST_TIMEOUT => Response::Standard(408, "Request Timeout").into_response(),
            HttpResponse::CONFLICT => Response::Standard(409, "Conflict").into_response(),
            HttpResponse::GONE => Response::Standard(410, "Gone").into_response(),
            HttpResponse::LENGTH_REQUIRED => Response::Standard(411, "Length Required").into_response(),
            HttpResponse::PRECONDITION_FAILED => Response::Standard(412, "Precondition Failed").into_response(),
            HttpResponse::REQUEST_ENTITY_TOO_LARGE => Response::Standard(413, "Request Entity Too Large").into_response(),
            HttpResponse::REQUEST_URI_TOO_LONG => Response::Standard(414, "Request-URI Too Long").into_response(),
            HttpResponse::UNSUPPORTED_MEDIA_TYPE => Response::Standard(415, "Unsupported Media Type").into_response(),
            HttpResponse::REQUESTED_RANGE_NOT_SATISFIABLE => Response::Standard(416, "Requested Range Not Satisfiable").into_response(),
            HttpResponse::EXPECTATION_FAILED => Response::Standard(417, "Expectation Failed").into_response(),
            HttpResponse::INTERNAL_SERVER_ERROR => Response::Standard(500, "Internal Server Error").into_response(),
            HttpResponse::NOT_IMPLEMENTED => Response::Standard(501, "Not Implemented").into_response(),
            HttpResponse::BAD_GATEWAY => Response::Standard(502, "Bad Gateway").into_response(),
            HttpResponse::SERVICE_UNAVAILABLE => Response::Standard(503, "Service Unavailable").into_response(),
            HttpResponse::GATEWAY_TIMEOUT => Response::Standard(504, "Gateway Timeout").into_response(),
            HttpResponse::HTTP_VERSION_NOT_SUPPORTED => Response::Standard(505, "HTTP Version Not Supported").into_response(),
            HttpResponse::CUSTOM(status, message) => Response::Standard(status, message).into_response(),
            HttpResponse::JSON(status, message, key, value) => Response::JsonData(status, message, key, value).into_response(),
        }
    }
}
