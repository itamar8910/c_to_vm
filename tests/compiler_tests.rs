
#[macro_use]
extern crate itertools;

extern crate simple_vm;
use simple_vm::operating_system::OS;
use simple_vm::operating_system::compiler::Compiler;

use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::io::Read;

const compiler_tests_dir : &str = "tests/compiler_test_data/";

#[derive(Debug)]
struct CompilerTestCase{
    category: String,
    input_f: String,
    target_f : String,
}

fn get_sorted_files(dir: &Path) -> Vec<String>{
        if dir.is_dir(){
            let mut files = Vec::new();
            for entry in fs::read_dir(dir).unwrap(){
                let entry = entry.unwrap();
                files.push(entry.path().to_str().unwrap().to_string());
            }
            files.sort();
            files
        }else{
            panic!();
        }
}

fn get_cases() -> Vec<CompilerTestCase>{
    let mut cases = Vec::new();
    let tests_dir = Path::new(&compiler_tests_dir);
    for entry in fs::read_dir(tests_dir).unwrap(){
        let entry = entry.unwrap();
        let path = entry.path();
        let category = path.file_name().unwrap().to_str().unwrap().to_string();
        if path.is_dir(){
            if !category.starts_with("_"){
                let c_files = get_sorted_files(&path.join("inputs"));
                let tar_files = get_sorted_files(&path.join("targets"));
                for (c_file, tar_file) in izip!(&c_files, &tar_files){
                    cases.push(CompilerTestCase{
                        category: category.clone(),
                        input_f: c_file.clone(),
                        target_f: tar_file.clone(),
                    })
                }
            }
        }else{
            panic!();
        }
    }
    cases
}

#[test]
fn test_compile_programs(){
    let cases = get_cases();
    for test_case in &cases{
        let program = Compiler::compile(&test_case.input_f);
        let mut os = OS::new();
        let res = os.assemble_and_run(&program);
        let mut tar_f = File::open(&test_case.target_f).unwrap();
        let mut tar_content = String::new();
        tar_f.read_to_string(&mut tar_content).unwrap();
        println!("{}", program);
        println!("{}:{}", test_case.category, Path::new(&test_case.input_f).file_name().unwrap().to_str().unwrap());
        assert_eq!(res.to_string(), tar_content.trim());
    }
}
