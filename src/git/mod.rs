pub mod branch;
pub mod repo;
pub mod http;


pub trait GitObject{
    fn get_id(&self) -> String;
}