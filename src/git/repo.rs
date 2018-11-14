use std::process::Command;
use std::process::Stdio;
use std::io::Write;
use std::process::ExitStatus;

pub struct Repository{
    pub path:String
}

pub fn git_init(path:&str) -> ExitStatus{
    let status = Command::new("git")
                .arg("init")
                .arg("--bare")
                .arg(path)
                .status()
                .expect("failed to execute process");
    return status;
}
