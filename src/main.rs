extern crate actix_web;
use actix_web::{App, fs, server, HttpRequest, HttpResponse};

use std::collections::HashMap;

pub mod template;

fn index(req: &HttpRequest) -> HttpResponse {
    let path = "public/index.tpl".to_string();
    let mut contents = template::read(&path, &HashMap::new());
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
        App::new()
            .resource("/", |r| r.f(index))
            .handler("/logo",
            fs::StaticFiles::new("public/logo").unwrap().show_files_listing())
            .resource("/{name}", |r| r.f(index)),
    ]})
    .bind("127.0.0.1:8088")
    .unwrap()
    .run();
}