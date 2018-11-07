use std::collections::HashMap;

use actix_web::{Path, Query, Form, HttpRequest, HttpResponse};
use actix_web::middleware::session::{Session, RequestSession};

use regex::Regex;

use tera::Context;

use uuid::Uuid;

use super::TERA;
use super::AppEnv;

use super::models::User;
use super::models::insert_user;
use super::models::find_user_by_email;
use super::models::find_user_by_fullname;

fn session_to_context(session:&Session) -> Context{
    let mut context = Context::new();

    let properties = vec!["uuid",
                        "user_name",
                        "user_fullname",
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
        let mut context = session_to_context(&req.session());
        // check in db
        let user = find_user_by_email(&state.connection, &form["email"]);

        match user{
            Some(v) => {
                if v.user_password==form["password"]{
                    // if ok save in session
                    req.session().set("uuid", &v.uuid).unwrap();
                    req.session().set("user_name", &v.user_name).unwrap();
                    req.session().set("user_fullname", &v.user_fullname).unwrap();
                    req.session().set("user_email", &v.user_email).unwrap();
                    
                    HttpResponse::Found().header("Location", "/").finish()
                }else{
                    context.insert("error_header", "Sign in error.");
                    context.insert("error_content", "Incorrect username or password.");
                    let contents = TERA.render("signin.html", &context).unwrap();
                    HttpResponse::Ok().body(&contents)
                }
            },
            None => {
                context.insert("error_header", "Sign in error.");
                context.insert("error_content", "Incorrect username or password.");
                let contents = TERA.render("signin.html", &context).unwrap();
                HttpResponse::Ok().body(&contents)
            }
        }
    }else{
        HttpResponse::BadRequest().finish()
    }
}


pub fn signout_action(req: &HttpRequest<AppEnv>) -> HttpResponse {
    let uuid:Option<String> = req.session().get("uuid").unwrap();
    if uuid.is_some(){
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
            user_fullname:form["name"].clone(), 
            user_email:form["email"].clone(), 
            user_password:form["password"].clone()
        });
        let mut context = session_to_context(&req.session());
        context.insert("message_header", "Your user registration was successful.");
        context.insert("message_content", "You may now log-in with the username you have chosen");
        let contents = TERA.render("signin.html", &context).unwrap();
        HttpResponse::Ok().body(&contents)
    }else{
        HttpResponse::BadRequest().finish()
    }
}


pub fn profile((req, path, query):(HttpRequest<AppEnv>, Path<(String,)>, Query<HashMap<String, String>>)) -> HttpResponse {
    let state = req.state();
    let mut context = session_to_context(&req.session());
    let user_fullname = &path.0;

    if let Some(cur_user) = find_user_by_fullname(&state.connection, user_fullname){
        context.insert("cur_user_name", &cur_user.user_name);
        context.insert("cur_user_fullname", &cur_user.user_fullname);
    }else{
        return HttpResponse::Ok().body("user no find");
    }

    

    let path = match query.get("tab"){
        None => "overview.html",
        Some(caps) => {
            if caps == "overview" {
                "overview.html"
            }else if caps == "repositories" {
                "repositories.html"
            }else if caps == "stars" {
                "stars.html"
            }else if caps == "followers" {
                "followers.html"
            }else if caps == "following" {
                "following.html"
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