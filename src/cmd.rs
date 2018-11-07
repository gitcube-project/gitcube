use std::process::Command;

fn git_init(path:&str){
    Command::new("git init")
            .arg(path)
            .spawn()
            .expect("failed to execute process");
}