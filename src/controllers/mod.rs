use tera::Context;
use actix_web::middleware::session::{Session};

pub mod home;
pub mod user;
pub mod git;
pub mod repo;

pub fn session_to_context(session:&Session) -> Context{
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