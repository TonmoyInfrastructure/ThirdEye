/*
* Documentation: https://crates.io/crates/actix-web
*/
/*
* App: This struct represents an Actix Web application and is used to configure routes and other common application settings.
* HttpServer: This struct represents an HTTP server instance and is used to instantiate and configure servers.
* web: This module provides essential types for route registration as well as common utilities for request handlers.
* HttpRequest and HttpResponse: These structs represent HTTP requests and responses and expose methods for creating, inspecting, and otherwise utilizing them. 
*/
use actix_web ::{
    get,
    web,
    App,
    HttpServer,
    Responder,
    http::header::ContentType,
    httpRequest,
    HttpResponse,
};
/*
* Documentation: https://crates.io/crates/tokio
*/
use tokio::fs::read_to_string;
/* 
* Documentation: https://docs.rs/tokio/latest/tokio/fs/fn.read_to_string.html#
*/

/*
* Creates a future which will open a file for reading and read the entire contents into a string and return said string.
* This is the async equivalent of `std::fs::read_to_string`.
* This operation is implemented by running the equivalent blocking operation on a separate thread pool using `spawn_blocking`. 
*/

#[get("/")]
pub async fn index(config: web::Data<&'static Config>) -> HttpResponse {
    let index_content = views::index::index(
        &config.style.colorscheme,
        &config.style.theme,
        &config.style.animation,
    );
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(index_content.0)
}
