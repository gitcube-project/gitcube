use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;

extern crate regex;
use regex::Regex;
use regex::Captures;

pub fn read_template(path: &'static str) -> String{
    let mut f = File::open(path).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents
}

pub fn read(path: &String) -> String{
    expand_extend(path)
}

pub fn expand_extend(path: &String) -> String{
    let mut f = File::open(path).expect("file not found");
    let file_path = Path::new(&path);
    let parent_path = file_path.parent().unwrap();

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    
    let extends_re = Regex::new(r"@extends\('(?P<extend_name>.*)'\)").unwrap();

    let result = extends_re.replace_all(&contents, |caps: &Captures| {
        let mut pathbuf = PathBuf::new();
        pathbuf.push(parent_path.to_str().unwrap());
        pathbuf.push(&caps["extend_name"]);
        let pathbuf_string = pathbuf.to_str().unwrap().to_string();
        println!("loading parent template:'{}'", pathbuf_string);
        expand_extend(&pathbuf_string)
    });

    result.to_string()
}

pub fn to_html(source: &String, hash_map: &HashMap<String, String>) -> String{
    let mut html = source.clone();

    html
}