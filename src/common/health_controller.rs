use okapi::openapi3::OpenApi;
use rocket_okapi::{openapi, openapi_get_routes_spec};
use rocket_okapi::settings::OpenApiSettings;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: health, liveness]
}

#[openapi(tag = "健康检查")]
#[get("/health")]
pub fn health() -> String {
    "OK".to_string()
}

#[openapi(tag = "健康检查")]
#[get("/liveness")]
pub fn liveness() -> String {
    "OK".to_string()
}



