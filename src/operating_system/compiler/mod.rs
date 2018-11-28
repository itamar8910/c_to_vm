extern crate serde_json;

use std::process::Command;

const PATH_TO_PY_EXEC : &str = "src/operating_system/compiler/parser/venv/bin/python";
const PATH_TO_PARSER : &str = "src/operating_system/compiler/parser/to_ast_json.py";

pub fn get_ast_json(path_to_c_source: &str) -> serde_json::Value{
    let output = Command::new(PATH_TO_PY_EXEC)
                        .arg(PATH_TO_PARSER)
                        .arg(path_to_c_source)
                        .output()
                        .expect("Failed to execute c parser");

    let json_str = String::from_utf8(output.stdout).expect("Error decoding ast json bytes");

    serde_json::from_str(&json_str).expect("parser output is not JSON serializable")
}
// fn compile(text: &str) -> Vec<String>{
//     let code: Vec<String> = Vec::new();
// }