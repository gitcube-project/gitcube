use std::process::Command;
use std::process::Stdio;


use super::repo::Repository;
use super::GitObject;

const BRANCH_PREFIX:&'static str = "refs/heads/";

#[derive(Serialize, Deserialize)]
pub struct Branch{
    name:String,
    id:String
}

impl GitObject for Branch{
    fn get_id(&self) -> &str{
        &self.id
    }

    fn get_name(&self) -> &str{
        &self.name
    }
}

pub trait BranchExt{
    fn get_branches(&self) -> Vec<Branch>;
    fn delete_branch(&self, branch_name:&str, is_force:bool);
}

impl BranchExt for Repository{
    fn delete_branch(&self, branch_name:&str, is_force:bool){
        if is_force {
            Command::new("git")
            .arg("branch")
            .arg("-D")
            .arg(branch_name)
            .current_dir(&self.path)
            .spawn()
            .expect("failed to execute process");
        }else{
            Command::new("git")
            .arg("branch")
            .arg("-d")
            .arg(branch_name)
            .current_dir(&self.path)
            .spawn()
            .expect("failed to execute process");
        }
    }
    fn get_branches(&self) -> Vec<Branch>{
        let output = Command::new("git")
            .arg("show-ref")
            .arg("--heads")
            .current_dir(&self.path)
            .output()
            .expect("failed to execute process");
        let output_str = String::from_utf8_lossy(output.stdout.as_slice());
        let lines = output_str.lines();
        let mut rst = Vec::new();
        for line in lines{
            let fields = line.split_whitespace().collect::<Vec<_>>();
            if fields.len()<2{
                continue;
            }

            rst.push(Branch{
                name: fields[1].trim_start_matches(BRANCH_PREFIX).to_string(),
                id: fields[0].to_string()
            });
        }
        rst
    }
}