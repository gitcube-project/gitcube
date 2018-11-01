use std::collections::HashMap;

use actix_web::{State, Form, HttpRequest, HttpResponse};
use actix_web::middleware::session::{RequestSession};

use regex::Regex;

use tera::Context;

use super::TERA;
use super::AppEnv;

use diesel::prelude::*;

use super::models::{User, NewUser};
use super::schema::users;
use super::schema::users::dsl::*;


pub fn index(req: &HttpRequest<AppEnv>) -> HttpResponse {
    let mut context = Context::new();
    if let Some(email) = req.session().get::<String>("email").unwrap(){
        context.insert("email", &email);
    }
    let contents = TERA.render("index.html", &context).unwrap();
    HttpResponse::Ok()
        .content_type("text/html")
        .body(&contents)
}

pub fn signin_page(req: &HttpRequest<AppEnv>) -> HttpResponse {
    let mut context = Context::new();
    if let Some(email) = req.session().get::<String>("email").unwrap(){
        context.insert("email", &email);
    }
    let contents = TERA.render("signin.html", &context).unwrap();
    HttpResponse::Ok().body(&contents)
}

pub fn signin_action((req, form): (HttpRequest<AppEnv>, Form<HashMap<String, String>>)) -> HttpResponse {
    if form.contains_key("email") && form.contains_key("password"){
        // check in db

        // if ok save in session
        req.session().set("email", form["email"].clone()).unwrap();
        
        HttpResponse::TemporaryRedirect().header("Location", "/").finish()
    }else{
        HttpResponse::BadRequest().finish()
    }
    
}


pub fn signout_action(req: &HttpRequest<AppEnv>) -> HttpResponse {
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

pub fn signup(req: &HttpRequest<AppEnv>) -> HttpResponse {
    let state = req.state();
    let new_user = NewUser{user_name: "123".to_string(),
                            user_email: "123@qq.com".to_string(),
                            user_password: "123123123".to_string()};
    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&state.connection).unwrap();
           
    let mut context = Context::new();
    if let Some(email) = req.session().get::<String>("email").unwrap(){
        context.insert("email", &email);
    }
    let contents = TERA.render("signup.html", &context).unwrap();
    HttpResponse::Ok()
        .content_type("text/html")
        .body(&contents)
}

pub fn profile(req: &HttpRequest<AppEnv>) -> HttpResponse {
    let mut context = Context::new();
    if let Some(email) = req.session().get::<String>("email").unwrap(){
        context.insert("email", &email);
    }
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

    let contents = TERA.render(path, &context).unwrap();
    
    HttpResponse::Ok()
        .content_type("text/html")
        .body(&contents)
}