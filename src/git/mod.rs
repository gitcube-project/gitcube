pub mod branch;
pub mod repo;
pub mod http;
pub mod tree;


pub trait GitObject{
    fn get_id(&self) -> &str;
    fn get_name(&self) -> &str;
}