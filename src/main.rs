extern crate actix_web;
use actix_web::{App, server, HttpRequest, HttpResponse, http::ContentEncoding};

pub mod template;

fn index(req: &HttpRequest) -> HttpResponse {
    let mut contents = template::read_template("public/index.tpl");
    HttpResponse::Ok()
        .content_type("text/html")
        .body(contents)
}

fn main() {
    server::new(|| {vec![
        App::new()
            .prefix("/user")
            .resource("/signin", |r| r.f(|r| HttpResponse::Ok()))
            .resource("/signup", |r| r.f(|r| HttpResponse::Ok())),
        App::new().resource("/", |r| r.f(index)),
    ]})
    .bind("127.0.0.1:8088")
    .unwrap()
    .run();
}