use std::collections::HashMap;

use uuid::Uuid;

use actix_web::{Form, Path, HttpRequest, HttpResponse};
use actix_web::middleware::session::{RequestSession};

use ::TERA;
use ::AppEnv;
use super::session_to_context;

use ::models::user::User;
use ::models::user::UserType;
use ::models::user::insert_user;

pub fn new_organization_page(req: &HttpRequest<AppEnv>) -> HttpResponse {
    let context = session_to_context(&req.session());
    let contents = TERA.render("new_organization.html", &context).unwrap();
    HttpResponse::Ok().body(&contents)
}

pub fn new_organization_action((req, form): (HttpRequest<AppEnv>, Form<HashMap<String, String>>)) -> HttpResponse {
    let state = req.state();
    if form.contains_key("org_name")
        && form.contains_key("description"){
        let user = req.session().get::<User>("user").unwrap().unwrap();
        let uuid = user.uuid;
        let user_fullname = user.fullname;
        // insert to db
        insert_user(&state.connection, &User{
            uuid:Uuid::new_v4().to_hyphenated().to_string(),
            name:form["org_name"].clone(), 
            fullname:form["org_name"].clone(), 
            email:String::from(""), 
            password:String::from(""),
            is_block:0,
            avatar:"/avatar/default.png".to_string(),
            type_id:UserType::Org.to_i32()
        });
        HttpResponse::Found().header("Location", format!("/{}", &form["org_name"])).finish()
    }else{
        HttpResponse::BadRequest().finish()
    }
}
/*
pub fn org_page((req, path): (HttpRequest<AppEnv>, Path<(String,String)>)) -> HttpResponse {
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
*/