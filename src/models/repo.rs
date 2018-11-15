
use ::error::Error;
use mysql::chrono::prelude::NaiveDateTime;
use super::Connection;

#[derive(Serialize, Deserialize)]
pub struct Repo{
    pub uuid:String,
    pub name:String,
    pub description:String,
    pub owner_uuid:String,
    pub create_time:String,
    pub is_private:i32,
    pub is_fork:i32,
    pub fork_uuid:String
}


pub fn insert_repo(connection:&Connection, repo:&Repo){
    match connection{
        Connection::Mysql(conn)=>{
            let mut stmt_insert = conn.prepare(r"INSERT INTO repo
                                       (uuid, name, description, owner_uuid, create_time, is_private, is_fork, fork_uuid)
                                        VALUES
                                       (:uuid, :name, :description, :owner_uuid, :create_time, :is_private, :is_fork, :fork_uuid)").unwrap();
            stmt_insert.execute(params!{
                    "uuid" => &repo.uuid,
                    "name" => &repo.name,
                    "description" => &repo.description,
                    "owner_uuid" => &repo.owner_uuid,
                    "create_time" => &repo.create_time,
                    "is_private" => &repo.is_private,
                    "is_fork" => &repo.is_fork,
                    "fork_uuid" => &repo.fork_uuid
                }).unwrap();
        }
    }
}

pub fn find_repo_by_username_reponame(connection:&Connection, user_fullname:&String, repo_name:&String) -> Option<Repo>{
    match connection{
        Connection::Mysql(conn)=>{
            let mut stmt = conn.prepare(r"SELECT repo.uuid, repo.name, repo.description, repo.owner_uuid, repo.create_time,
                                                repo.is_private, repo.is_fork, repo.fork_uuid
                                                FROM repo
                                                LEFT JOIN user
                                                ON repo.owner_uuid=user.uuid
                                                WHERE user.fullname=:user_fullname AND
                                                repo.name=:repo_name").unwrap();
            let row = stmt.execute(params!{
                    "user_fullname" => user_fullname,
                    "repo_name" => repo_name
                }).unwrap().last();
            
            match row{
                Some(v)=>{
                    let repo = v.unwrap();
                    Some(Repo{
                        uuid:repo.get(0).unwrap(),
                        name:repo.get(1).unwrap(),
                        description:repo.get(2).unwrap(),
                        owner_uuid:repo.get(3).unwrap(),
                        create_time:NaiveDateTime::from(repo.get(4).unwrap()).format("%Y-%m-%d %H:%M:%S").to_string(),
                        is_private:repo.get(5).unwrap(),
                        is_fork:repo.get(6).unwrap(),
                        fork_uuid:repo.get(7).unwrap()
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
            let mut stmt_insert = conn.prepare(r"SELECT uuid, name, description, owner_uuid, create_time, is_private, is_fork, fork_uuid
                                                FROM repo
                                                WHERE owner_uuid=:owner_uuid").unwrap();
            let rows = stmt_insert.execute(params!{
                    "owner_uuid" => uuid
                }).unwrap();
            
            let mut rst = Vec::new();
            for row in rows {
                let repo = row.unwrap();
                rst.push(Repo{
                        uuid:repo.get(0).unwrap(),
                        name:repo.get(1).unwrap(),
                        description:repo.get(2).unwrap(),
                        owner_uuid:repo.get(3).unwrap(),
                        create_time:NaiveDateTime::from(repo.get(4).unwrap()).format("%Y-%m-%d %H:%M:%S").to_string(),
                        is_private:repo.get(5).unwrap(),
                        is_fork:repo.get(6).unwrap(),
                        fork_uuid:repo.get(7).unwrap()
                    });
            }
            rst
        }
    }
}


pub fn get_star_by_repo_uuid(connection:&Connection, uuid:&String) -> Result<i32, Error>{
    match connection{
        Connection::Mysql(conn)=>{
            let mut stmt = conn.prepare(r"SELECT COUNT(repo.uuid)
                FROM repo
                LEFT JOIN star
                ON repo.uuid == star.repo_uuid
                WHERE repo.uuid=:uuid")?;
            let row = stmt.execute(params!{
                    "uuid" => uuid,
                })?.last();
            
            match row{
                Some(v)=>{
                    let record = v?;
                    Ok(record.get(0).unwrap_or(0))
                },
                None=> Ok(0)
            }
        }
    }
}

pub fn get_watch_by_repo_uuid(connection:&Connection, uuid:&String) -> Result<i32, Error>{
    match connection{
        Connection::Mysql(conn)=>{
            let mut stmt = conn.prepare(r"SELECT COUNT(repo.uuid)
                FROM repo
                LEFT JOIN watch
                ON repo.uuid == watch.repo_uuid
                WHERE repo.uuid=:uuid")?;
            let row = stmt.execute(params!{
                    "uuid" => uuid,
                })?.last();
            
            match row{
                Some(v)=>{
                    let record = v?;
                    Ok(record.get(0).unwrap_or(0))
                },
                None=> Ok(0)
            }
        }
    }
}