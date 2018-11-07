
use actix_web::{HttpRequest, HttpResponse};
use actix_web::middleware::session::{RequestSession};

use super::super::TERA;
use super::super::AppEnv;
use super::session_to_context;

pub fn index(req: &HttpRequest<AppEnv>) -> HttpResponse {
    let context = session_to_context(&req.session());
    let contents = TERA.render("index.html", &context).unwrap();
    HttpResponse::Ok()
        .content_type("text/html")
        .body(&contents)
}