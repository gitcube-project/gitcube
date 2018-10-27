use std::env;
use std::fs::File;
use std::io::prelude::*;

extern crate regex;
use regex::Regex;

pub fn read_template(path: &'static str) -> String{
    let mut f = File::open(path).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents
}

pub fn to_html(source: &String) -> String{
    let mut html = source.clone();

    html
}