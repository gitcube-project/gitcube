
pub enum Connection {
    Mysql(mysql::Pool),
}

impl Connection{
    pub fn new_mysql(url:&str) -> Connection{
        Connection::Mysql(mysql::Pool::new(url)
                    .expect(&format!("Error connecting to {}", url)),
        )
    }
}

pub struct User{
    pub uuid:String,
    pub user_name:String,
    pub user_fullname:String,
    pub user_email:String,
    pub user_password:String,
}

pub struct Repo{
    pub uuid:String,
    pub repo_name:String,
    pub repo_description:String,
    pub repo_owner_uuid:String,
    pub repo_create_time:String,
}

pub fn insert_user(connection:&Connection, user:&User){
    match connection{
        Connection::Mysql(conn)=>{
            let mut stmt_insert = conn.prepare(r"INSERT INTO users
                                       (uuid, user_name, user_fullname, user_email, user_password)
                                        VALUES
                                       (:uuid, :user_name, :user_fullname, :user_email, :user_password)").unwrap();
            stmt_insert.execute(params!{
                    "uuid" => &user.uuid,
                    "user_name" => &user.user_name,
                    "user_fullname" => &user.user_fullname,
                    "user_email" => &user.user_email,
                    "user_password" => &user.user_password,
                }).unwrap();
        }
    }
}

pub fn find_user_by_fullname(connection:&Connection, user_fullname:&String)->Option<User>{
    match connection{
        Connection::Mysql(conn)=>{
            let mut stmt_insert = conn.prepare(r"SELECT uuid, user_name, user_fullname, user_email, user_password
                                                FROM users
                                                WHERE user_fullname=:user_fullname").unwrap();
            let row = stmt_insert.execute(params!{
                    "user_fullname" => user_fullname
                }).unwrap().last();
                
            match row{
                Some(v)=>{
                    let user = v.unwrap();
                    Some(User{
                        uuid:user.get(0).unwrap(),
                        user_name:user.get(1).unwrap(),
                        user_fullname:user.get(2).unwrap(),
                        user_email:user.get(3).unwrap(),
                        user_password:user.get(4).unwrap(),
                    })},
                None=>None
            }
        }
    }
}

pub fn find_user_by_uuid(connection:&Connection, uuid:&String)->Option<User>{
    match connection{
        Connection::Mysql(conn)=>{
            let mut stmt_insert = conn.prepare(r"SELECT (uuid, user_name, user_fullname, user_email, user_password)
                                                FROM users
                                                WHERE uuid=:uuid").unwrap();
            let row = stmt_insert.execute(params!{
                    "uuid" => uuid
                }).unwrap().last();
                
            match row{
                Some(v)=>{
                    let user = v.unwrap();
                    Some(User{
                        uuid:user.get(0).unwrap(),
                        user_name:user.get(1).unwrap(),
                        user_fullname:user.get(2).unwrap(),
                        user_email:user.get(3).unwrap(),
                        user_password:user.get(4).unwrap(),
                    })},
                None=>None
            }
        }
    }
}

pub fn find_user_by_email(connection:&Connection, email:&String)->Option<User>{
    match connection{
        Connection::Mysql(conn)=>{
            let mut stmt_insert = conn.prepare(r"SELECT uuid, user_name, user_fullname, user_email, user_password
                                                FROM users
                                                WHERE user_email=:email").unwrap();
            let row = stmt_insert.execute(params!{
                    "email" => email
                }).unwrap().last();
                
            match row{
                Some(v)=>{
                    let user = v.unwrap();
                    Some(User{
                        uuid:user.get(0).unwrap(),
                        user_name:user.get(1).unwrap(),
                        user_fullname:user.get(2).unwrap(),
                        user_email:user.get(3).unwrap(),
                        user_password:user.get(4).unwrap(),
                    })},
                None=>None
            }
        }
    }
}