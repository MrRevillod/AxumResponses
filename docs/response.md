## Response

The `Response` struct represents a standard HTTP response that can be used in Axum handlers.

It implements `IntoResponse` to convert
the response into an Axum-compatible response.

The IntoResponse returns a `axum::response::Response` that follows two conventions:

1.  `RFC 7231` - HTTP/1.1 Semantics and Content
2.  `RFC 8259` - The JavaScript Object Notation (JSON) Data Interchange Format

The headers are stored in a `HeaderMap`
but they are not serialized into the final JSON body.

## Http Code Variants

The struct provides methods for common HTTP status codes for example:

- `HttpResponse::Ok()` for 200 OK
- `HttpResponse::NotFound()` for 404 Not Found

## Response Body

Represents the json body of the generated HTTP response.
Can be used to tests to verify the structure of the response.

The `data`, `error` and `errors` fields are optional and will be included
only if is setted in the `Response` builder.

## `response!` Macro

This macro is used to create a response builder with a specified status code.
It's a shorthand for constructing an `Response` with a status code.

It can also be used to create a response with a JSON body.
The macro takes a status code as an argument, and optionally a JSON object.

If a JSON object is provided, it will be serialized and included in the response body `data` key.

If the JSON object contains a `message` key, it will be extracted and set as the response message and removed from the data object.
