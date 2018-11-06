
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

pub fn insert_user(connection:&Connection, user_name:&String, user_email:&String, user_password:&String){
    match connection{
        Connection::Mysql(conn)=>{
            let mut stmt_insert = conn.prepare(r"INSERT INTO users
                                       (user_name, user_email, user_password)
                                        VALUES
                                       (:user_name, :user_email, :user_password)").unwrap();
            stmt_insert.execute(params!{
                    "user_name" => user_name,
                    "user_email" => user_email,
                    "user_password" => user_password,
                }).unwrap();
        }
    }
    
}