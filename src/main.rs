#[macro_use] extern crate rocket;
use utoipa::{OpenApi, ToSchema};
use rocket::serde::{Serialize, json::Json};

// Define the API documentation structure
#[derive(OpenApi)]
#[openapi(
    paths(nms),
    components(schemas(ApiResponse))
)]
struct ApiDoc;

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
        (status = 200, description = "Successful response", body = ApiResponse)
    )
)]
#[get("/nms")]
fn nms() -> Json<ApiResponse> {
    Json(ApiResponse {
        message: "Hello, NMS".to_owned()
    })
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