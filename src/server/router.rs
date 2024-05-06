/**
* Documentation: https://crates.io/crates/actix-web
**/
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

