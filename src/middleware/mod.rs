use tera::Context;
use std::env;
use actix_web::middleware::session::{Session};

pub mod auth;

pub fn env_to_context(session:&Session) -> Context{
    let mut context = Context::new();

    if let Some(v) = session.get::<::models::user::User>("user").unwrap(){
        context.insert("user", &v);
    }

    let root_url = env::var("ROOT_URL").expect("ROOT_URL must be set");
    context.insert("ROOT_URL", &root_url);

    context
}