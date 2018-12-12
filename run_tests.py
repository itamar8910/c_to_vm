#!/usr/bin/python3

import subprocess
from os import path, listdir

"""
auto-generates test cases for compiler
then runs cargo-test

(couldn't get rust's macro system to create a function from a macro expression (only from an identifer))
"""

TESTS_DIR = 'tests/compiler_test_data/'
COMPILER_TESTS_FILE = 'tests/generated_compiler_tests.rs'

def get_test_categories():
    return {
        category: {
            'inputs': sorted(listdir(path.join(TESTS_DIR, category, 'inputs'))),
            'targets': sorted(listdir(path.join(TESTS_DIR, category, 'targets'))),
        } for category in listdir(TESTS_DIR) if not category.startswith('_')
    }


test_cases = get_test_categories()

categories, inputs, targets = zip(*[(category, inp, tar) for category, data in test_cases.items() for inp, tar in zip(data['inputs'], data['targets']) if not inp.startswith('_')])
assert all([i.replace('.c', '') == t.replace('.txt', '') for i, t in zip(inputs, targets)]), 'input/target mismatch!'


compiler_tests_code = """

// This file was auto-generated by run_tests.py
// test cases are from: https://github.com/nlsandler/write_a_c_compiler/

extern crate simple_vm;
use simple_vm::operating_system::OS;
use simple_vm::operating_system::compiler::Compiler;

use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::io::Read;

#[derive(Debug)]
struct CompilerTestCase{
    category: String,
    input_f: String,
    target_f : String,
}

fn test_single(test_case: &CompilerTestCase){
    println!("{}:{},{}", test_case.category, Path::new(&test_case.input_f).file_name().unwrap().to_str().unwrap(), Path::new(&test_case.target_f).file_name().unwrap().to_str().unwrap());
    let program = Compiler::compile(&test_case.input_f);
    let mut os = OS::new();
    let res = os.assemble_and_run(&program);
    let mut tar_f = File::open(&test_case.target_f).unwrap();
    let mut tar_content = String::new();
    tar_f.read_to_string(&mut tar_content).unwrap();
    println!("{}", program);
    println!("{},{}", res.to_string(), tar_content.to_string());
    assert_eq!(res.to_string(), tar_content.trim());
}
"""

for cat, inp, tar in zip(categories, inputs, targets):
    compiler_tests_code += \
    f"""
#[test]
fn test_{cat}_{path.splitext(path.basename(inp))[0]}(){{
    let case = CompilerTestCase{{
        category: "{cat}".to_string(),
        input_f: "{path.join(TESTS_DIR, cat, 'inputs', inp)}".to_string(),
        target_f: "{path.join(TESTS_DIR, cat, 'targets', tar)}".to_string(),
    }};
    test_single(&case);
}}
    """


with open(COMPILER_TESTS_FILE, 'w') as f:
    f.write(compiler_tests_code)
# print(compiler_tests_code)

subprocess.run("cargo test", shell=True)