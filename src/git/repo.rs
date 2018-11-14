use std::process::Command;
use std::process::Stdio;
use std::io::Write;
use std::process::ExitStatus;

pub struct Repository{
    pub path:String
}


pub fn init_repository(path:&str) -> Result<Repository, &str>{
    let status = Command::new("git")
                .arg("init")
                .arg("--bare")
                .arg(path)
                .status()
                .expect("failed to execute process");
    
    match status.success(){
        true => Ok(Repository{path:path.to_string()}),
        false => Err("failed to execute process")
    }
}

pub fn open_repository(path:&str) -> Result<Repository, &str>{
    let p = std::path::Path::new(path);
    if p.exists() && p.is_dir(){
        Ok(Repository{
            path:path.to_string()
        })
    }else{
        Err("Git repository no found")
    }
}


