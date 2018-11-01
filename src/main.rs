

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

pub mod controllers;

lazy_static! {
    pub static ref TERA: Tera = {
        let mut tera = compile_templates!("public/templates/**/*");
        // and we can add more things to our instance if we want to
        tera.autoescape_on(vec!["html"]);
        tera
    };
}

fn main() {
    server::new(|| App::new()
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
                r.method(Method::POST).f(controllers::signout_action)
            })
            .resource("/signup", |r| r.f(controllers::signup))
            .resource("", |r| r.f(controllers::index))
            .resource("/", |r| r.f(controllers::index))
            .resource("/{name:[0-9a-zA-Z]+}", |r| r.f(controllers::profile))
    )
    .bind("127.0.0.1:8088")
    .unwrap()
    .run();
}