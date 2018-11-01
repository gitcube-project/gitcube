

extern crate actix_web;
use actix_web::{App, http::Method, fs, server};
use actix_web::middleware::session::{SessionStorage, CookieSessionBackend};


extern crate regex;


#[macro_use]
extern crate tera;
use tera::Tera;

#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use dotenv::dotenv;
use std::env;

pub mod controllers;
pub mod models;
pub mod schema;

lazy_static! {
    pub static ref TERA: Tera = {
        let mut tera = compile_templates!("public/templates/**/*");
        // and we can add more things to our instance if we want to
        tera.autoescape_on(vec!["html"]);
        tera
    };
}

pub struct AppEnv {
    connection: MysqlConnection,
}



fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn main() {
    server::new(|| App
            ::with_state(AppEnv { connection: establish_connection() })
            .middleware(SessionStorage::new(
                CookieSessionBackend::signed(&[0; 32]).secure(false)
            ))
            .handler("/logo",
            fs::StaticFiles::new("public/logo").unwrap().show_files_listing())
            .handler("/assets",
            fs::StaticFiles::new("public/assets").unwrap().show_files_listing())
            .handler("/js",
            fs::StaticFiles::new("public/assets").unwrap().show_files_listing())
            .handler("/css",
            fs::StaticFiles::new("public/assets").unwrap().show_files_listing())
            .resource("/signin", |r|{
                r.method(Method::GET).f(controllers::signin_page);
                r.method(Method::POST).with(controllers::signin_action)
            })
            .resource("/signout", |r|{
                r.method(Method::GET).f(controllers::signout_action)
            })
            .resource("/signup", |r|{
                r.method(Method::GET).f(controllers::signup_page);
                r.method(Method::POST).with(controllers::signup_action)
            })
            .resource("", |r| r.f(controllers::index))
            .resource("/", |r| r.f(controllers::index))
            .resource("/{name:[0-9a-zA-Z]+}", |r| r.f(controllers::profile))
    )
    .bind("127.0.0.1:8088")
    .unwrap()
    .run();
}