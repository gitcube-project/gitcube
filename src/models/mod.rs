
pub mod repo;
pub mod user;


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