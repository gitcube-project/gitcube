use std::collections::HashMap;

use uuid::Uuid;

use actix_web::{Form, HttpRequest, HttpResponse};
use actix_web::middleware::session::{RequestSession};

use ::TERA;
use ::AppEnv;
use super::session_to_context;

use ::models::repo::Repo;
use ::models::repo::insert_repo;

pub fn new_repository_page(req: &HttpRequest<AppEnv>) -> HttpResponse {
    let mut context = session_to_context(&req.session());
    let mut available_owners = Vec::new();
    if let Some(v) = req.session().get::<String>("user_fullname").unwrap(){
        available_owners.push(v);
    }
    context.insert("available_owners", &available_owners);
    let contents = TERA.render("new_repository.html", &context).unwrap();
    HttpResponse::Ok().body(&contents)
}

pub fn new_repository_action((req, form): (HttpRequest<AppEnv>, Form<HashMap<String, String>>)) -> HttpResponse {
    let state = req.state();
    if form.contains_key("owner")
        && form.contains_key("repo_name")
        && form.contains_key("private")
        && form.contains_key("description"){
        let uuid = req.session().get::<String>("uuid").unwrap().unwrap();
        let user_fullname = req.session().get::<String>("user_fullname").unwrap().unwrap();
        let context = session_to_context(&req.session());
        // insert to db
        insert_repo(&state.connection, &Repo{
            uuid:Uuid::new_v4().to_hyphenated().to_string(),
            repo_name:form["repo_name"].clone(), 
            repo_description:form["description"].clone(), 
            repo_owner_uuid:uuid.clone(), 
            repo_create_time:chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()
        });
        // run git cmd

        HttpResponse::Found().header("Location", format!("/{}/{}",&user_fullname, &form["repo_name"])).finish()
    }else{
        HttpResponse::BadRequest().finish()
    }
}

pub fn repo_page(req: &HttpRequest<AppEnv>) -> HttpResponse {
    let context = session_to_context(&req.session());
    let contents = TERA.render("repository.html", &context).unwrap();
    HttpResponse::Ok().body(&contents)
}