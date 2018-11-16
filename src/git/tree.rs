use std::process::Command;
use std::process::Stdio;

use ::git::repo::Repository;

#[derive(Serialize, Deserialize)]
pub enum EntryMode{
    Blob,
	Exec,
	Symlink,
	Commit,
	Tree,
}


impl EntryMode{
    pub fn to_i32(&self) -> i32{
        match self{
            EntryMode::Blob => 0100644,
            EntryMode::Exec => 0100755,
            EntryMode::Symlink => 0120000,
            EntryMode::Commit => 0160000,
            EntryMode::Tree => 0040000,
        }
    }

    pub fn from_i32(code:i32) -> EntryMode{
        if code == 0100644{
            EntryMode::Blob
        }else if code == 0100755{
            EntryMode::Exec
        }else if code == 0120000{
            EntryMode::Symlink
        }else if code == 0160000{
            EntryMode::Commit
        }else if code == 0040000{
            EntryMode::Tree
        }else{
            EntryMode::Blob
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum ObjectType{
    Commit,
	Tree,
	Blob,
	Tag,
}


impl ObjectType{
    pub fn to_string(&self) -> &str{
        match self{
            ObjectType::Commit => "commit",
            ObjectType::Tree => "tree",
            ObjectType::Blob => "blob",
            ObjectType::Tag => "tag",
        }
    }

    pub fn from_string(s:&str) -> ObjectType{
        if s=="commit"{
            ObjectType::Commit
        }else if s=="tree"{
            ObjectType::Tree
        }else if s=="blob"{
            ObjectType::Blob
        }else if s=="tag"{
            ObjectType::Tag
        }else{
            ObjectType::Commit
        }
    }
}


#[derive(Serialize, Deserialize)]
pub struct TreeEntry{
    id:String,
	object_type:ObjectType,
	entry_mode:EntryMode,
	name:String,
	commited:bool,
	size:i64
}

impl TreeEntry{
    pub fn name(&self) -> &str{
        return &self.name;
    }

    pub fn size(&self) -> i64{
        if self.is_dir() {
            return 0;
        } else {
            return self.size;
        }
    }

    pub fn is_dir(&self) -> bool {
        match self.entry_mode{
            EntryMode::Tree => true,
            _ => false
        }
    }

    pub fn is_link(&self) -> bool {
        match self.entry_mode{
            EntryMode::Symlink => true,
            _ => false
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Tree{
    id: String,
	entries: Vec<TreeEntry>
}

pub trait TreeExt{
    fn get_tree(&self, commit: &str, path:&str) -> Option<Tree>;
}

impl TreeExt for Repository{
    fn get_tree(&self, commit: &str, tree_path:&str) -> Option<Tree>{
        let output = Command::new("git")
            .arg("ls-tree")
            .arg(commit)
            .current_dir(&self.path)
            .output()
            .expect("failed to execute process");
        let output_str = String::from_utf8_lossy(output.stdout.as_slice());
        let lines = output_str.lines();
        let mut rst = Tree{
            id:commit.to_string(),
            entries: Vec::new()
        };
        for line in lines{
            let fields = line.split_whitespace().collect::<Vec<_>>();
            if fields.len()<4{
                continue;
            }

            rst.entries.push(TreeEntry{
                id:fields[2].to_string(),
                object_type: ObjectType::from_string(fields[1]),
                entry_mode: EntryMode::from_i32(fields[0].parse::<i32>().unwrap()),
                name:fields[3].to_string(),
                commited:true,
                size:0
            });
        }
        Some(rst)
    }
}