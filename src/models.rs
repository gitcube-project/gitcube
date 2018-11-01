use super::schema::users;
use super::Queryable;
use super::Insertable;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub user_name: String,
    pub user_email: String,
    pub user_password: String,
}


#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub user_name: String,
    pub user_email: String,
    pub user_password: String,
}