
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
    pub user_email:String,
    pub user_password:String,
}

pub fn insert_user(connection:&Connection, user:&User){
    match connection{
        Connection::Mysql(conn)=>{
            let mut stmt_insert = conn.prepare(r"INSERT INTO users
                                       (uuid, user_name, user_email, user_password)
                                        VALUES
                                       (:uuid, :user_name, :user_email, :user_password)").unwrap();
            stmt_insert.execute(params!{
                    "uuid" => &user.uuid,
                    "user_name" => &user.user_name,
                    "user_email" => &user.user_email,
                    "user_password" => &user.user_password,
                }).unwrap();
        }
    }
}

pub fn find_user_by_name(connection:&Connection, user_name:&String)->Option<User>{
    match connection{
        Connection::Mysql(conn)=>{
            let mut stmt_insert = conn.prepare(r"SELECT (uuid, user_name, user_email, user_password)
                                                FROM users
                                                WHERE user_name=:user_name").unwrap();
            let row = stmt_insert.execute(params!{
                    "user_name" => user_name
                }).unwrap().last();
                
            match row{
                Some(v)=>{
                    let user = v.unwrap();
                    Some(User{
                        uuid:user.get(0).unwrap(),
                        user_name:user.get(1).unwrap(),
                        user_email:user.get(2).unwrap(),
                        user_password:user.get(3).unwrap(),
                    })},
                None=>None
            }
        }
    }
}