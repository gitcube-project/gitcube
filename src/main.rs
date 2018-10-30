extern crate actix_web;
use actix_web::{App, fs, server, HttpRequest, HttpResponse};

use std::collections::HashMap;

extern crate regex;
use regex::Regex;

pub mod template;

fn index(req: &HttpRequest) -> HttpResponse {
    let path = "public/index.tpl".to_string();
    let contents = template::read(&path, &HashMap::new());
    HttpResponse::Ok()
        .content_type("text/html")
        .body(contents)
}

fn signin(req: &HttpRequest) -> HttpResponse {
    let path = "public/signin.tpl".to_string();
    let contents = template::read(&path, &HashMap::new());
    HttpResponse::Ok()
        .content_type("text/html")
        .body(contents)
}

fn signup(req: &HttpRequest) -> HttpResponse {
    let path = "public/signup.tpl".to_string();
    let contents = template::read(&path, &HashMap::new());
    HttpResponse::Ok()
        .content_type("text/html")
        .body(contents)
}

fn profile(req: &HttpRequest) -> HttpResponse {
    let query = match req.uri().query(){
        None => "",
        Some(q) => q
    };
    let re = Regex::new(r"(?:(?:&|^)tab=(?P<tab>[a-zA-Z]+)(?:&|$))").unwrap();
    let path = match re.captures(query){
        None => "public/overview.tpl".to_string(),
        Some(caps) => {
            if &caps["tab"]=="overview" {
                "public/overview.tpl".to_string()
            }else if &caps["tab"]=="repositories" {
                "public/repositories.tpl".to_string()
            }else if &caps["tab"]=="stars" {
                "public/stars.tpl".to_string()
            }else if &caps["tab"]=="followers" {
                "public/followers.tpl".to_string()
            }else if &caps["tab"]=="followering" {
                "public/followering.tpl".to_string()
            }else{
                "public/overview.tpl".to_string()
            }
        }
    };

    let contents = template::read(&path, &HashMap::new());
    
    HttpResponse::Ok()
        .content_type("text/html")
        .body(contents)
}

fn main() {
    server::new(|| {vec![
        App::new()
            .prefix("/user")
            .resource("/signin", |r| r.f(signin))
            .resource("/signup", |r| r.f(signup)),
        App::new()
            .resource("/", |r| r.f(index))
            .handler("/logo",
            fs::StaticFiles::new("public/logo").unwrap().show_files_listing())
            .handler("/assets",
            fs::StaticFiles::new("public/assets").unwrap().show_files_listing())
            .handler("/js",
            fs::StaticFiles::new("public/assets").unwrap().show_files_listing())
            .handler("/css",
            fs::StaticFiles::new("public/assets").unwrap().show_files_listing())
            .resource("/{name:[0-9a-zA-Z]+}", |r| r.f(profile)),
    ]})
    .bind("127.0.0.1:8088")
    .unwrap()
    .run();
}