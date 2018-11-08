use std::collections::HashMap;

use uuid::Uuid;

use actix_web::{http::Method, Path, Query, Binary, HttpRequest, HttpResponse, HttpMessage};
use actix_web::middleware::session::{RequestSession};

use super::super::AppEnv;
use ::cmd::git_upload_pack_adverise_refs;
use ::cmd::git_receive_pack_adverise_refs;
use ::cmd::git_upload_pack;
use ::cmd::git_receive_pack;

pub fn git_advertise_refs((req, path, query):(HttpRequest<AppEnv>, Path<(String,String)>, Query<HashMap<String, String>>)) -> HttpResponse {
    let repo_path = &format!("{git}/{user}/{repo}",
            git = std::env::var("GIT_PATH").expect("GIT_PATH must be set"),
            user = &path.0,
            repo = &path.1);
    if let Some(service) = query.get("service"){
        if service=="git-upload-pack"{
            let git_ret = git_upload_pack_adverise_refs(&repo_path);
            let packet = b"# service=git-upload-pack\n";
            let hex = format!("{:>04x}", packet.len() + 4);

            let mut body:Vec<u8> = Vec::new();
            body.extend(hex.as_bytes());
            body.extend(packet);
            body.extend(b"0000");
            body.extend(&git_ret);
            HttpResponse::Ok()
                .header("Expires", "Fri, 01 Jan 1980 00:00:00 GMT")
                .header("Pragma", "no-cache")
                .header("Cache-Control", "no-cache, max-age=0, must-revalidate")
                .header("Content-Type", "application/x-git-upload-pack-advertisement")
                .body(body)

        }else if service=="git-receive-pack"{
            let git_ret = git_receive_pack_adverise_refs(&repo_path);
            let packet = b"# service=git-receive-pack\n";
            let hex = format!("{:>04x}", packet.len() + 4);

            let mut body:Vec<u8> = Vec::new();
            body.extend(hex.as_bytes());
            body.extend(packet);
            body.extend(b"0000");
            body.extend(&git_ret);
            HttpResponse::Ok()
                .header("Expires", "Fri, 01 Jan 1980 00:00:00 GMT")
                .header("Pragma", "no-cache")
                .header("Cache-Control", "no-cache, max-age=0, must-revalidate")
                .header("Content-Type", "application/x-git-receive-pack-advertisement")
                .body(body)
        }else{
            HttpResponse::BadRequest().finish()
        }
    }else{
        HttpResponse::BadRequest().finish()
    }
}

pub fn git_upload_pack_handler((req, path, body):(HttpRequest<AppEnv>, Path<(String,String)>, bytes::Bytes)) -> HttpResponse {
    let repo_path = &format!("{git}/{user}/{repo}",
            git = std::env::var("GIT_PATH").expect("GIT_PATH must be set"),
            user = &path.0,
            repo = &path.1);
    let ret_val = git_upload_pack(&repo_path, &body);
    HttpResponse::Ok()
        .header("Expires", "Fri, 01 Jan 1980 00:00:00 GMT")
        .header("Pragma", "no-cache")
        .header("Cache-Control", "no-cache, max-age=0, must-revalidate")
        .header("Content-Type", "application/x-git-upload-pack-result")
        .body(ret_val)
}

pub fn git_receive_pack_handler((req, path, body):(HttpRequest<AppEnv>, Path<(String,String)>, bytes::Bytes)) -> HttpResponse {
    let repo_path = &format!("{git}/{user}/{repo}",
            git = std::env::var("GIT_PATH").expect("GIT_PATH must be set"),
            user = &path.0,
            repo = &path.1);
    let ret_val = git_receive_pack(&repo_path, &body);
    HttpResponse::Ok()
        .header("Expires", "Fri, 01 Jan 1980 00:00:00 GMT")
        .header("Pragma", "no-cache")
        .header("Cache-Control", "no-cache, max-age=0, must-revalidate")
        .header("Content-Type", "application/x-git-receive-pack-result")
        .body(ret_val)
}