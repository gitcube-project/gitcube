use actix_web::http::{header, HttpTryFrom};
use actix_web::{App, HttpRequest, HttpResponse, Result};
use actix_web::middleware::{Middleware, Started, Response};
use actix_web::middleware::session::{RequestSession};

use ::TERA;
use ::AppEnv;
use super::env_to_context;

use ::models::user::User;

use regex::RegexSet;

pub struct Auth;

impl<S> Middleware<S> for Auth {
    fn start(&self, req: &HttpRequest<S>) -> Result<Started> {
        if is_limit(req) && !is_auth(req){
            let context = env_to_context(&req.session());
            let contents = TERA.render("404.html", &context).unwrap();
            Ok(Started::Response(
                HttpResponse::Ok().body(&contents)
            ))
        }else{
            Ok(Started::Done)
        }
    }
}

fn is_limit<S>(req: &HttpRequest<S>) -> bool{
    let set = RegexSet::new(&[
        r"^/$",
        r"^/signin$",
        r"^/signup$",
        r"^/help$",
        r"^/[a-zA-Z_]+$",
        r"^/[a-zA-Z_]+/[a-zA-Z_]+$",
    ]).unwrap();
    
    return !set.is_match(req.path());
}

fn is_auth<S>(req: &HttpRequest<S>) -> bool{
    if let Some(_user) = req.session().get::<User>("user").unwrap(){
        true
    }else{
        false
    }
}