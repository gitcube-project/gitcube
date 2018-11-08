use std::collections::HashMap;

use uuid::Uuid;

use actix_web::{Form, Path, HttpRequest, HttpResponse};
use actix_web::middleware::session::{RequestSession};

use ::TERA;
use ::AppEnv;
use super::session_to_context;

use ::models::repo::Repo;
use ::models::repo::insert_repo;

use ::cmd::git_init;

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
        git_init(&format!("{git}/{user}/{repo}",
            git = std::env::var("GIT_PATH").expect("GIT_PATH must be set"),
            user = &user_fullname,
            repo = &form["repo_name"]));
        HttpResponse::Found().header("Location", format!("/{}/{}",&user_fullname, &form["repo_name"])).finish()
    }else{
        HttpResponse::BadRequest().finish()
    }
}

pub fn repo_page((req, path): (HttpRequest<AppEnv>, Path<(String,String)>)) -> HttpResponse {
    let mut context = session_to_context(&req.session());
    context.insert("cur_user_fullname", &path.0);
    context.insert("cur_repo_name", &path.1);
    let contents = TERA.render("repository.html", &context).unwrap();
    HttpResponse::Ok().body(&contents)
}