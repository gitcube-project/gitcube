
use super::Connection;

pub struct User{
    pub uuid:String,
    pub name:String,
    pub fullname:String,
    pub email:String,
    pub password:String,
    pub is_block:i32
}


pub fn insert_user(connection:&Connection, user:&User){
    match connection{
        Connection::Mysql(conn)=>{
            let mut stmt_insert = conn.prepare(r"INSERT INTO users
                                       (uuid, name, fullname, email, password, is_block)
                                        VALUES
                                       (:uuid, :name, :fullname, :email, :password)").unwrap();
            stmt_insert.execute(params!{
                    "uuid" => &user.uuid,
                    "name" => &user.name,
                    "fullname" => &user.fullname,
                    "email" => &user.email,
                    "password" => &user.password,
                    "is_block" => &user.is_block
                }).unwrap();
        }
    }
}


pub fn find_user_by_fullname(connection:&Connection, fullname:&String)->Option<User>{
    match connection{
        Connection::Mysql(conn)=>{
            let mut stmt = conn.prepare(r"SELECT uuid, name, fullname, email, password, is_block
                                                FROM users
                                                WHERE fullname=:fullname").unwrap();
            let row = stmt.execute(params!{
                    "fullname" => fullname
                }).unwrap().last();
                
            match row{
                Some(v)=>{
                    let user = v.unwrap();
                    Some(User{
                        uuid:user.get(0).unwrap(),
                        name:user.get(1).unwrap(),
                        fullname:user.get(2).unwrap(),
                        email:user.get(3).unwrap(),
                        password:user.get(4).unwrap(),
                        is_block:user.get(5).unwrap()
                    })},
                None=>None
            }
        }
    }
}

pub fn find_user_by_uuid(connection:&Connection, uuid:&String)->Option<User>{
    match connection{
        Connection::Mysql(conn)=>{
            let mut stmt = conn.prepare(r"SELECT (uuid, name, fullname, email, password, is_block)
                                                FROM users
                                                WHERE uuid=:uuid").unwrap();
            let row = stmt.execute(params!{
                    "uuid" => uuid
                }).unwrap().last();
                
            match row{
                Some(v)=>{
                    let user = v.unwrap();
                    Some(User{
                        uuid:user.get(0).unwrap(),
                        name:user.get(1).unwrap(),
                        fullname:user.get(2).unwrap(),
                        email:user.get(3).unwrap(),
                        password:user.get(4).unwrap(),
                        is_block:user.get(5).unwrap()
                    })},
                None=>None
            }
        }
    }
}

pub fn find_user_by_email(connection:&Connection, email:&String)->Option<User>{
    match connection{
        Connection::Mysql(conn)=>{
            let mut stmt_insert = conn.prepare(r"SELECT uuid, name, fullname, email, password, is_block
                                                FROM users
                                                WHERE email=:email").unwrap();
            let row = stmt_insert.execute(params!{
                    "email" => email
                }).unwrap().last();
                
            match row{
                Some(v)=>{
                    let user = v.unwrap();
                    Some(User{
                        uuid:user.get(0).unwrap(),
                        name:user.get(1).unwrap(),
                        fullname:user.get(2).unwrap(),
                        email:user.get(3).unwrap(),
                        password:user.get(4).unwrap(),
                        is_block:user.get(5).unwrap()
                    })},
                None=>None
            }
        }
    }
}