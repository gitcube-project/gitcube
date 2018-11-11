use std::collections::HashMap;

use uuid::Uuid;

use actix_web::{Form, Path, HttpRequest, HttpResponse};
use actix_web::middleware::session::{RequestSession};

use ::TERA;
use ::AppEnv;
use super::session_to_context;

use ::models::repo::Repo;
use ::models::repo::insert_repo;
use ::models::repo::find_repo_by_username_reponame;

use ::git::repo::git_init;

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
        // let context = session_to_context(&req.session());
        // insert to db
        let repo_uuid = Uuid::new_v4().to_hyphenated().to_string();
        insert_repo(&state.connection, &Repo{
            uuid:repo_uuid.clone(),
            name:form["repo_name"].clone(), 
            description:form["description"].clone(), 
            owner_uuid:uuid.clone(), 
            create_time:chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            is_private:0,
            is_fork:0,
            fork_uuid:Uuid::nil().to_hyphenated().to_string()
        });
        // run git cmd
        git_init(&format!("{git}/{uuid}",
            git = std::env::var("GIT_PATH").expect("GIT_PATH must be set"),
            uuid = &repo_uuid));
        HttpResponse::Found().header("Location", format!("/{}/{}",&user_fullname, &form["repo_name"])).finish()
    }else{
        HttpResponse::BadRequest().finish()
    }
}

pub fn repo_page((req, path): (HttpRequest<AppEnv>, Path<(String,String)>)) -> HttpResponse {
    let state = req.state();
    let repo_opt = find_repo_by_username_reponame(&state.connection, &path.0, &path.1);

    if repo_opt.is_none(){
        return HttpResponse::BadRequest().finish();
    }
    
    let mut context = session_to_context(&req.session());
    context.insert("cur_user_fullname", &path.0);
    context.insert("cur_repo_name", &path.1);
    let contents = TERA.render("repository.html", &context).unwrap();
    HttpResponse::Ok().body(&contents)
}