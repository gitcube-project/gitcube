extern crate uuid;
extern crate bytes;
extern crate chrono;

extern crate argparse;
use argparse::{ArgumentParser, StoreTrue, Store};

#[macro_use]
extern crate log;
extern crate fern;
use log::{info, trace, warn};

extern crate actix_web;
use actix_web::{App, http::Method, fs, server};
use actix_web::middleware::session::{SessionStorage, CookieSessionBackend};


extern crate regex;


#[macro_use]
extern crate tera;
use tera::Tera;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;


#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate mysql;
extern crate dotenv;

use dotenv::dotenv;
use std::env;

pub mod controllers;
pub mod models;
pub mod git;
pub mod error;

use models::Connection;

lazy_static! {
    pub static ref TERA: Tera = {
        let mut tera = compile_templates!("templates/**/*");
        // and we can add more things to our instance if we want to
        tera.autoescape_on(vec!["html"]);
        tera
    };
}

pub struct AppEnv {
    connection: Connection,
}

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}

fn establish_connection() -> Connection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    Connection::new_mysql(&database_url)
}

fn start_server(){
    info!("Web server is starting.");
    server::new(|| App
            ::with_state(AppEnv { connection: establish_connection() })
            .middleware(SessionStorage::new(
                CookieSessionBackend::signed(&[0; 32]).secure(false)
            ))
            .handler("/logo", fs::StaticFiles::new("public/logo").unwrap().show_files_listing())
            .handler("/assets", fs::StaticFiles::new("public/assets").unwrap().show_files_listing())
            .handler("/avatar", fs::StaticFiles::new("data/avatar").unwrap().show_files_listing())
            .resource("/signin", |r|{
                r.method(Method::GET).f(controllers::user::signin_page);
                r.method(Method::POST).with(controllers::user::signin_action)
            })
            .resource("/signout", |r|{
                r.method(Method::GET).f(controllers::user::signout_action)
            })
            .resource("/signup", |r|{
                r.method(Method::GET).f(controllers::user::signup_page);
                r.method(Method::POST).with(controllers::user::signup_action)
            })
            .resource("/new", |r|{
                r.method(Method::GET).f(controllers::repo::new_repository_page);
                r.method(Method::POST).with(controllers::repo::new_repository_action)
            })
            .resource("/organization/new", |r|{
                r.method(Method::GET).f(controllers::org::new_organization_page);
                r.method(Method::POST).with(controllers::org::new_organization_action)
            })
            .resource("/404", |r|{
                r.method(Method::GET).f(controllers::status::page_404)
            })
            .resource("/help", |r|{
                r.method(Method::GET).f(controllers::user::help_page)
            })
            .resource("", |r| r.f(controllers::home::index))
            .resource("/", |r| r.f(controllers::home::index))
            .resource("/{name:[0-9a-zA-Z]+}", |r| r.method(Method::GET).with(controllers::user::profile))
            .resource("/{name:[0-9a-zA-Z]+}/{repo:[0-9a-zA-Z]+}", |r| r.method(Method::GET).with(controllers::repo::repo_page))
            .resource("/{name:[0-9a-zA-Z]+}/{repo:[0-9a-zA-Z]+}.git/info/refs", |r| r.method(Method::GET).with(controllers::git::git_advertise_refs))
            .resource("/{name:[0-9a-zA-Z]+}/{repo:[0-9a-zA-Z]+}.git/git-upload-pack", |r| r.method(Method::POST).with(controllers::git::git_upload_pack_handler))
            .resource("/{name:[0-9a-zA-Z]+}/{repo:[0-9a-zA-Z]+}.git/git-receive-pack", |r| r.method(Method::POST).with(controllers::git::git_receive_pack_handler))
    )
    .bind("127.0.0.1:8088")
    .unwrap()
    .run();
}

fn main() {
    let mut web = true;
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("GitCube : A rust Git service.");
        ap.refer(&mut web)
            .add_option(&["-w", "--web"], StoreTrue,
            "Start web server");
        ap.parse_args_or_exit();
    }

    setup_logger().unwrap();

    if web{
        start_server();
    }
    
}