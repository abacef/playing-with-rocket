#[macro_use]
extern crate rocket;
use rocket::http::Status;
use rocket::serde::{json::Json, Serialize};
use utoipa::{OpenApi, ToSchema};

// Define the API documentation structure
#[derive(OpenApi)]
#[openapi(paths(nms), components(schemas(ApiResponse)))]
struct ApiDoc;

#[derive(ToSchema, Serialize)]
#[serde(crate = "rocket::serde")]
struct ApiError {
    error: String,
}

// Define a response schema for documentation
#[derive(ToSchema, Serialize)]
#[serde(crate = "rocket::serde")]
struct ApiResponse {
    message: String,
}

/// Get a greeting from NMS
///
/// Returns a simple greeting message.
#[utoipa::path(
    get,
    responses(
        (status = 200, description = "Successful response", body = ApiResponse),
        (status = 500, description = "Internal server error", body = ApiError)
    )
)]
#[get("/nms")]
fn nms() -> Result<Json<ApiResponse>, (Status, Json<ApiError>)> {
    // Simulating an error condition
    let some_error_condition = false;
    if some_error_condition {
        Err((
            Status::InternalServerError,
            Json(ApiError {
                error: "Internal server error".to_string(),
            }),
        ))
    } else {
        Ok(Json(ApiResponse {
            message: "Hello, NMS".to_owned(),
        }))
    }
}

#[get("/openapi_json")]
fn openapi_json() -> String {
    ApiDoc::openapi().to_pretty_json().unwrap()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![nms])
        .mount("/docs", routes![openapi_json])
}
