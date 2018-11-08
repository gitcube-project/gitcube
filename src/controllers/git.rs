use std::collections::HashMap;

use uuid::Uuid;

use actix_web::{http::Method, Path, Query, Binary, HttpRequest, HttpResponse, HttpMessage};
use actix_web::middleware::session::{RequestSession};

use super::super::AppEnv;
use ::cmd::git_upload_pack_adverise_refs;
use ::cmd::git_upload_pack;


pub fn git_advertise_refs((req, path, query):(HttpRequest<AppEnv>, Path<(String,)>, Query<HashMap<String, String>>)) -> HttpResponse {
    if let Some(service) = query.get("service"){
        if service=="git-upload-pack"{
            let git_ret = git_upload_pack_adverise_refs("git_repo/test");
            let packet = b"# service=git-upload-pack\n";

            let mut body:Vec<u8> = Vec::new();
            body.extend(b"001e");
            body.extend(packet);
            body.extend(b"0000");
            body.extend(&git_ret);
            HttpResponse::Ok()
                .header("Expires", "Fri, 01 Jan 1980 00:00:00 GMT")
                .header("Pragma", "no-cache")
                .header("Cache-Control", "no-cache, max-age=0, must-revalidate")
                .header("Content-Type", "application/x-git-upload-pack-advertisement")
                .body(body)
        }else{
            HttpResponse::BadRequest().finish()
        }
    }else{
        HttpResponse::BadRequest().finish()
    }
}

pub fn git_upload_pack_handler((req, body):(HttpRequest<AppEnv>, bytes::Bytes)) -> HttpResponse {
    let ret_val = git_upload_pack("git_repo/test", &body);
    HttpResponse::Ok()
        .header("Expires", "Fri, 01 Jan 1980 00:00:00 GMT")
        .header("Pragma", "no-cache")
        .header("Cache-Control", "no-cache, max-age=0, must-revalidate")
        .header("Content-Type", "application/x-git-upload-pack-result")
        .body(ret_val)
}