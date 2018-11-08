
use mysql::chrono::prelude::NaiveDateTime;
use super::Connection;

#[derive(Serialize, Deserialize)]
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

pub fn find_repo_by_username_reponame(connection:&Connection, user_fullname:&String, repo_name:&String) -> Option<Repo>{
    match connection{
        Connection::Mysql(conn)=>{
            let mut stmt = conn.prepare(r"SELECT repos.uuid, repo_name, repo_description, repo_owner_uuid, repo_create_time
                                                FROM repos
                                                LEFT JOIN users
                                                ON repos.repo_owner_uuid=users.uuid
                                                WHERE user_fullname=:user_fullname AND
                                                repo_name=:repo_name").unwrap();
            let row = stmt.execute(params!{
                    "user_fullname" => user_fullname,
                    "repo_name" => repo_name
                }).unwrap().last();
            
            match row{
                Some(v)=>{
                    let repo = v.unwrap();
                    Some(Repo{
                        uuid:repo.get(0).unwrap(),
                        repo_name:repo.get(1).unwrap(),
                        repo_description:repo.get(2).unwrap(),
                        repo_owner_uuid:repo.get(3).unwrap(),
                        repo_create_time:NaiveDateTime::from(repo.get(4).unwrap()).format("%Y-%m-%d %H:%M:%S").to_string(),
                    })
                },
                None=>None
            }
        }
    }
}

pub fn find_repo_by_user_uuid(connection:&Connection, uuid:&String) -> Vec<Repo>{
    match connection{
        Connection::Mysql(conn)=>{
            let mut stmt_insert = conn.prepare(r"SELECT uuid, repo_name, repo_description, repo_owner_uuid, repo_create_time
                                                FROM repos
                                                WHERE repo_owner_uuid=:repo_owner_uuid").unwrap();
            let rows = stmt_insert.execute(params!{
                    "repo_owner_uuid" => uuid
                }).unwrap();
            
            let mut rst = Vec::new();
            for row in rows {
                let repo = row.unwrap();
                rst.push(Repo{
                        uuid:repo.get(0).unwrap(),
                        repo_name:repo.get(1).unwrap(),
                        repo_description:repo.get(2).unwrap(),
                        repo_owner_uuid:repo.get(3).unwrap(),
                        repo_create_time:NaiveDateTime::from(repo.get(4).unwrap()).format("%Y-%m-%d %H:%M:%S").to_string(),
                    });
            }
            rst
        }
    }
}
