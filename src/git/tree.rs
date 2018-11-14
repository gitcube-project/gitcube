use std::process::Command;
use std::process::Stdio;


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
}

pub enum ObjectType{
    Commit,
	Tree,
	Blob,
	Tag,
}


impl ObjectType{
    pub fn to_string(&self) -> String{
        match self{
            ObjectType::Commit => "commit",
            ObjectType::Tree => "tree",
            ObjectType::Blob => "blob",
            ObjectType::Tag => "tag",
        }
    }
}



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
        return self.name;
    }

    pub fn size(&self) -> i64{
        if self.is_dir() {
            return 0;
        } else {
            return self.size;
        }
    }

    pub fn is_dir(&self) -> bool {
        return self.entry_mode == EntryMode::Tree;
    }

    pub fn is_link(&self) bool {
        return self.entry_mode == EntryMode::Symlink;
    }
}


pub struct Tree{
    
}