use std::collections::HashMap;

use uuid::Uuid;

use actix_web::{Path, Query, Form, HttpRequest, HttpResponse};
use actix_web::middleware::session::{RequestSession};

use super::super::TERA;
use super::super::AppEnv;
use super::session_to_context;



pub fn page_404(req: &HttpRequest<AppEnv>) -> HttpResponse {
    let context = session_to_context(&req.session());
    let contents = TERA.render("404.html", &context).unwrap();
    HttpResponse::Ok().body(&contents)
}