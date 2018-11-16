use std::collections::HashMap;

use uuid::Uuid;

use actix_web::{Form, Path, HttpRequest, HttpResponse};
use actix_web::middleware::session::{RequestSession};

use ::TERA;
use ::AppEnv;
use super::session_to_context;

use ::models::user::User;
use ::models::user::find_user_by_fullname;
use ::models::repo::Repo;
use ::models::repo::insert_repo;
use ::models::repo::find_repo_by_username_reponame;

use ::git::repo::Repository;
use ::git::repo::init_repository;
use ::git::repo::open_repository;
use ::git::branch::BranchExt;
use ::git::GitObject;
use ::git::tree::TreeExt;

pub fn new_repository_page(req: &HttpRequest<AppEnv>) -> HttpResponse {
    let mut context = session_to_context(&req.session());
    let mut available_owners = Vec::new();
    if let Some(v) = req.session().get::<User>("user").unwrap(){
        available_owners.push(v.fullname);
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
        let user = req.session().get::<User>("user").unwrap().unwrap();
        let uuid = user.uuid;
        let user_fullname = user.fullname;
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
        init_repository(&format!("{git}/{uuid}",
            git = std::env::var("GIT_PATH").expect("GIT_PATH must be set"),
            uuid = &repo_uuid)).unwrap();
        HttpResponse::Found().header("Location", format!("/{}/{}",&user_fullname, &form["repo_name"])).finish()
    }else{
        HttpResponse::BadRequest().finish()
    }
}

pub fn repo_page((req, path): (HttpRequest<AppEnv>, Path<(String,String)>)) -> HttpResponse {
    let state = req.state();
    let repo_opt = find_repo_by_username_reponame(&state.connection, &path.0, &path.1);
    let user_opt = find_user_by_fullname(&state.connection, &path.0);

    if repo_opt.is_none() || user_opt.is_none(){
        return HttpResponse::BadRequest().finish();
    }

    let repo = repo_opt.unwrap();
    let cur_user = user_opt.unwrap();

    // get branches
    let repo_obj = open_repository(
        &format!("{git}/{uuid}",
        git = std::env::var("GIT_PATH").expect("GIT_PATH must be set"),
        uuid = &repo.uuid)
    ).unwrap();

    // build context
    let mut context = session_to_context(&req.session());

    let branches = repo_obj.get_branches();
    if branches.len() != 0{
        let files = repo_obj.get_tree(branches[0].get_name(), "/");
        context.insert("files", &files);
    }
    
    context.insert("cur_user", &cur_user);
    context.insert("cur_repo", &repo);
    context.insert("branch_list", &branches);
    let contents = TERA.render("repository.html", &context).unwrap();
    HttpResponse::Ok().body(&contents)
}


pub fn star_repo((req, path): (HttpRequest<AppEnv>, Path<(String,String)>)) -> HttpResponse {
    HttpResponse::Found().header("Location", format!("/{}/{}",&path.0, &path.1)).finish()
}

pub fn watch_repo((req, path): (HttpRequest<AppEnv>, Path<(String,String)>)) -> HttpResponse {
    HttpResponse::Found().header("Location", format!("/{}/{}",&path.0, &path.1)).finish()
}
