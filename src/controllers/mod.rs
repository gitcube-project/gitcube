use tera::Context;
use std::env;
use actix_web::middleware::session::{Session};

pub mod home;
pub mod user;
pub mod git;
pub mod repo;
pub mod org;
pub mod status;

pub fn session_to_context(session:&Session) -> Context{
    let mut context = Context::new();

    // String session
    /*
    let properties = vec![""];
    for each in properties{
        if let Some(v) = session.get::<String>(each).unwrap(){
            context.insert(each, &v);
        }
    }
    */

    if let Some(v) = session.get::<::models::user::User>("user").unwrap(){
        context.insert("user", &v);
    }

    let root_url = env::var("ROOT_URL").expect("ROOT_URL must be set");
    context.insert("ROOT_URL", &root_url);

    context
}