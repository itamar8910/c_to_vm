extern crate regex;
use regex::Regex;


use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::ffi::OsStr;


static STD_DIR : &str = "./libc";

pub fn expand_include(include_str: &str, program_dir: &Path) -> Vec<String> {
    let include_program_path = program_dir.join(Path::new(include_str));
    let mut include_file = File::open(include_program_path.to_str().unwrap()).unwrap();
    let mut include_program = String::new(); 
    include_file.read_to_string(&mut include_program);
    include_program.split("\n").map(|s| s.to_string()).collect()
}

pub fn preprocess(program_path: &str) -> String{
    let program_dir = Path::new(program_path).parent().unwrap();
    let mut file = File::open(program_path).unwrap();
    let mut program = String::new();
    file.read_to_string(&mut program).unwrap();
    let src_lines: Vec<&str> = program.split("\n").collect();
    let mut dst_lines : Vec<String> = Vec::new();
    let include_re = Regex::new("^#include \"(.+)\"$").unwrap();
    let std_include_re = Regex::new("^#include <(.+)>$").unwrap();
    for line in src_lines.iter(){
        if let Some(caps) = include_re.captures(&line){
            dst_lines.append(&mut expand_include(&caps[1], program_dir));
        } else if let Some(caps) = std_include_re.captures(&line){
            dst_lines.append(&mut expand_include(&caps[1], Path::new(STD_DIR)));
        }
        else{
            dst_lines.push(line.clone().to_string());
        }
    } 
    dst_lines.join("\n")
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_include(){
        let program_path = "tests/preprocessor_test_data/include/main1.c";
        let result = preprocess(program_path);
        let mut target = String::new();
        let mut target_f = File::open("tests/preprocessor_test_data/include/tar.c").unwrap();
        target_f.read_to_string(&mut target);
        assert_eq!(result, target);
    }
}