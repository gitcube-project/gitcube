use std::collections::HashMap;

use actix_web::{Form, HttpRequest, HttpResponse};
use actix_web::middleware::session::{RequestSession};

use regex::Regex;

use tera::Context;

use super::TERA;

pub fn index(_req: &HttpRequest) -> HttpResponse {
    let contents = TERA.render("index.html", &json!({
        "name": "John Doe"
    })).unwrap();
    HttpResponse::Ok()
        .content_type("text/html")
        .body(&contents)
}

pub fn signin_page(_req: &HttpRequest) -> HttpResponse {
    let contents = TERA.render("signin.html", &json!({})).unwrap();
    HttpResponse::Ok().body(&contents)
}

pub fn signin_action((req, form): (HttpRequest, Form<HashMap<String, String>>)) -> HttpResponse {
    if form.contains_key("email") && form.contains_key("password"){
        // check in db

        // if ok save in session
        req.session().set("email", form["email"].clone()).unwrap();

        let mut context = Context::new();
        context.insert("email", &form["email"]);
        let contents = TERA.render("signin.html", &context).unwrap();
        
        HttpResponse::Ok().body(&contents)
    }else{
        HttpResponse::Ok().body("error parameters")
    }
    
}


pub fn signout_action(req: &HttpRequest) -> HttpResponse {
    let email:Option<String> = req.session().get("email").unwrap();
    if email.is_some(){
        req.session().remove("email");
        let context = Context::new();
        let contents = TERA.render("signin.html", &context).unwrap();
        req.session().set("context", context).unwrap();
        HttpResponse::Ok().body(&contents)
    }else{
        HttpResponse::Ok().body("error")
    }
}

pub fn signup(_req: &HttpRequest) -> HttpResponse {
    let contents = TERA.render("signup.html", &json!({
        "name": "John Doe"
    })).unwrap();
    HttpResponse::Ok()
        .content_type("text/html")
        .body(&contents)
}

pub fn profile(req: &HttpRequest) -> HttpResponse {
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