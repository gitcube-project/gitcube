extern crate actix_web;
use actix_web::{App, fs, server, HttpRequest, HttpResponse};

extern crate regex;
use regex::Regex;

#[macro_use]
extern crate tera;
use tera::Tera;
// use tera::Context;

#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    pub static ref TERA: Tera = {
        let mut tera = compile_templates!("public/templates/**/*");
        // and we can add more things to our instance if we want to
        tera.autoescape_on(vec!["html"]);
        tera
    };
}



fn index(req: &HttpRequest) -> HttpResponse {
    let contents = TERA.render("index.html", &json!({
        "name": "John Doe"
    })).unwrap();
    HttpResponse::Ok()
        .content_type("text/html")
        .body(&contents)
}

fn signin(req: &HttpRequest) -> HttpResponse {
    let contents = TERA.render("signin.html", &json!({
        "name": "John Doe"
    })).unwrap();
    HttpResponse::Ok()
        .content_type("text/html")
        .body(&contents)
}

fn signup(req: &HttpRequest) -> HttpResponse {
    let contents = TERA.render("signup.html", &json!({
        "name": "John Doe"
    })).unwrap();
    HttpResponse::Ok()
        .content_type("text/html")
        .body(&contents)
}

fn profile(req: &HttpRequest) -> HttpResponse {
    let query = match req.uri().query(){
        None => "",
        Some(q) => q
    };
    let re = Regex::new(r"(?:(?:&|^)tab=(?P<tab>[a-zA-Z]+)(?:&|$))").unwrap();
    let path = match re.captures(query){
        None => "overview.html",
        Some(caps) => {
            if &caps["tab"]=="overview" {
                "overview.html"
            }else if &caps["tab"]=="repositories" {
                "repositories.html"
            }else if &caps["tab"]=="stars" {
                "stars.html"
            }else if &caps["tab"]=="followers" {
                "followers.html"
            }else if &caps["tab"]=="followering" {
                "followering.html"
            }else{
                "overview.html"
            }
        }
    };

    let contents = TERA.render(path, &json!({
        "name": "John Doe"
    })).unwrap();
    
    HttpResponse::Ok()
        .content_type("text/html")
        .body(&contents)
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