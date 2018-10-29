use std::fs::File;
use std::io::prelude::*;
use std::vec::Vec;
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;

extern crate regex;
use regex::Regex;
use regex::Captures;
use regex::RegexBuilder;

pub fn read(path: &String, var_map: &HashMap<String,String>) -> String{
    println!("loading template:'{}'", path);
    let expand_tpl = expand_extend(path);
    let section_replaced_tpl = section_replace(&expand_tpl);
    replace_variables(&section_replaced_tpl, var_map)
}

fn expand_extend(path: &String) -> String{
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

fn section_replace(source: &String) -> String{
    let mut section_list: Vec<(String,String)> = Vec::new();
    let mut rst = String::new();

    // get all sections
    {
        let mut builder = RegexBuilder::new(r"@section\('(?P<section_name>.*?)'\)(?P<section_content>.*?)@endsection");
        builder.multi_line(true);
        builder.dot_matches_new_line(true);
        let re = builder.build().unwrap();

        let result = re.replace_all(&source, |caps: &Captures| {
            println!("section:'{}'", caps["section_name"].to_string());
            section_list.push((caps["section_name"].to_string(), caps["section_content"].to_string()));
            ""
        });
        rst = result.to_string();
    }

    // replace all yield
    {
        for (key, value) in &section_list{
            let re = Regex::new(&format!(r"@yield\('{section_name}'\)", section_name = &key)).unwrap();
            rst = re.replace(&rst, value.as_str()).to_string();
        }
    }

    // replace remained yield
    {
        let re = Regex::new(r"@yield\('.*?'\)").unwrap();
        rst = re.replace_all(&rst, "").to_string();
    }

    return rst;
}

fn replace_variables(source: &String, hash_map: &HashMap<String, String>) -> String{
    let mut html = source.clone();
    for (key, value) in hash_map{
        html = html.replace(&format!("{{{name}}}", name=key.as_str()), value.as_str());
    }
    return html;
}