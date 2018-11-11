use super::Connection;

#[derive(Serialize, Deserialize)]
pub struct Org{
    pub uuid:String,
    pub name:String,
    pub description:String,
}


pub fn insert_org(connection:&Connection, org:&Org){
    match connection{
        Connection::Mysql(conn)=>{
            let mut stmt_insert = conn.prepare(r"INSERT INTO orgs
                                       (uuid, name, description)
                                        VALUES
                                       (:uuid, :name, :description)").unwrap();
            stmt_insert.execute(params!{
                    "uuid" => &org.uuid,
                    "name" => &org.name,
                    "description" => &org.description,
                }).unwrap();
        }
    }
}