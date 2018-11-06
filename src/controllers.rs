use std::collections::HashMap;

use actix_web::{Form, HttpRequest, HttpResponse};
use actix_web::middleware::session::{Session, RequestSession};

use regex::Regex;

use tera::Context;

use uuid::Uuid;

use super::TERA;
use super::AppEnv;

use super::models::User;
use super::models::insert_user;
use super::models::find_user_by_email;

fn session_to_context(session:&Session) -> Context{
    let mut context = Context::new();

    let properties = vec!["uuid",
                        "user_name",
                        "user_email"];
    for each in properties{
        if let Some(v) = session.get::<String>(each).unwrap(){
            context.insert(each, &v);
        }
    }
    context
}

pub fn index(req: &HttpRequest<AppEnv>) -> HttpResponse {
    let context = session_to_context(&req.session());
    let contents = TERA.render("index.html", &context).unwrap();
    HttpResponse::Ok()
        .content_type("text/html")
        .body(&contents)
}

pub fn signin_page(req: &HttpRequest<AppEnv>) -> HttpResponse {
    let context = session_to_context(&req.session());
    let contents = TERA.render("signin.html", &context).unwrap();
    HttpResponse::Ok().body(&contents)
}

pub fn signin_action((req, form): (HttpRequest<AppEnv>, Form<HashMap<String, String>>)) -> HttpResponse {
    let state = req.state();
    if form.contains_key("email") && form.contains_key("password"){
        // check in db
        let user = find_user_by_email(&state.connection, &form["email"]);

        match user{
            Some(v) => {
                if v.user_password==form["password"]{
                    // if ok save in session
                    req.session().set("uuid", &v.uuid).unwrap();
                    req.session().set("user_name", &v.user_name).unwrap();
                    req.session().set("user_email", &v.user_email).unwrap();
                    HttpResponse::Found().header("Location", "/").finish()
                }else{
                    HttpResponse::Found().header("Location", "/").finish()
                }
            },
            None => {
                HttpResponse::Found().header("Location", "/").finish()
            }
        }
    }else{
        HttpResponse::BadRequest().finish()
    }
}


pub fn signout_action(req: &HttpRequest<AppEnv>) -> HttpResponse {
    let email:Option<String> = req.session().get("email").unwrap();
    if email.is_some(){
        req.session().remove("uuid");
        req.session().remove("user_name");
        req.session().remove("user_email");
        req.session().clear();
        let context = session_to_context(&req.session());
        let contents = TERA.render("signin.html", &context).unwrap();
        HttpResponse::Ok().body(&contents)
    }else{
        HttpResponse::Ok().body("error")
    }
}

pub fn signup_page(req: &HttpRequest<AppEnv>) -> HttpResponse {
    let context = session_to_context(&req.session());
    let contents = TERA.render("signup.html", &context).unwrap();
    HttpResponse::Ok()
        .content_type("text/html")
        .body(&contents)
}

pub fn signup_action((req, form): (HttpRequest<AppEnv>, Form<HashMap<String, String>>)) -> HttpResponse {
    let state = req.state();
    if form.contains_key("name") && 
    form.contains_key("email") &&
    form.contains_key("password"){
        insert_user(&state.connection, &User{
            uuid:Uuid::new_v4().to_hyphenated().to_string(),
            user_name:form["name"].clone(), 
            user_email:form["email"].clone(), 
            user_password:form["password"].clone()
        });
        HttpResponse::Found().header("Location", "/signin").finish()
    }else{
        HttpResponse::BadRequest().finish()
    }
}


pub fn profile(req: &HttpRequest<AppEnv>) -> HttpResponse {
    let context = session_to_context(&req.session());
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