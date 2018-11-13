use std::process::Command;
use std::process::Stdio;
use std::io::Write;

pub struct Repository{
    pub path:String
}

pub fn git_init(path:&str){
    Command::new("git")
            .arg("init")
            .arg("--bare")
            .arg(path)
            .spawn()
            .expect("failed to execute process");
}
