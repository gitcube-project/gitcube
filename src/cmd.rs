use std::process::Command;
use std::process::Stdio;
use std::io::Write;

pub fn git_init(path:&str){
    Command::new("git")
            .arg("init")
            .arg("--bare")
            .arg(path)
            .spawn()
            .expect("failed to execute process");
}

pub fn git_upload_pack_adverise_refs(path:&str) -> Vec<u8>{
    let output = Command::new("git")
            .arg("upload-pack")
            .arg("--stateless-rpc")
            .arg("--advertise-refs")
            .arg(path)
            .output()
            .expect("failed to execute process");
    output.stdout.as_slice().to_vec()
}

pub fn git_receive_pack_adverise_refs(path:&str) -> Vec<u8>{
    let output = Command::new("git")
            .arg("receive-pack")
            .arg("--stateless-rpc")
            .arg("--advertise-refs")
            .arg(path)
            .output()
            .expect("failed to execute process");
    output.stdout.as_slice().to_vec()
}

pub fn git_upload_pack(path:&str, in_str: &bytes::Bytes) -> Vec<u8>{
    let mut child = Command::new("git")
            .arg("upload-pack")
            .arg("--stateless-rpc")
            .arg(path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to execute process");
    {
        let mut stdin = child.stdin.as_mut().expect("Failed to open stdin");
        stdin.write_all(in_str).expect("Failed to write to stdin");
    }
    let output = child.wait_with_output().expect("Failed to read stdout");
    output.stdout.to_vec()
}

pub fn git_receive_pack(path:&str, in_str: &bytes::Bytes) -> Vec<u8>{
    let mut child = Command::new("git")
            .arg("receive-pack")
            .arg("--stateless-rpc")
            .arg(path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to execute process");
    {
        let mut stdin = child.stdin.as_mut().expect("Failed to open stdin");
        stdin.write_all(in_str).expect("Failed to write to stdin");
    }
    let output = child.wait_with_output().expect("Failed to read stdout");
    output.stdout.to_vec()
}