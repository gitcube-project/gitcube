

use super::Connection;

pub struct Repo{
    pub uuid:String,
    pub repo_name:String,
    pub repo_description:String,
    pub repo_owner_uuid:String,
    pub repo_create_time:String,
}


pub fn insert_repo(connection:&Connection, repo:&Repo){
    match connection{
        Connection::Mysql(conn)=>{
            let mut stmt_insert = conn.prepare(r"INSERT INTO repos
                                       (uuid, repo_name, repo_description, repo_owner_uuid, repo_create_time)
                                        VALUES
                                       (:uuid, :repo_name, :repo_description, :repo_owner_uuid, :repo_create_time)").unwrap();
            stmt_insert.execute(params!{
                    "uuid" => &repo.uuid,
                    "repo_name" => &repo.repo_name,
                    "repo_description" => &repo.repo_description,
                    "repo_owner_uuid" => &repo.repo_owner_uuid,
                    "repo_create_time" => &repo.repo_create_time,
                }).unwrap();
        }
    }
}
