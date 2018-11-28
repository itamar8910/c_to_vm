extern crate serde_json;

use std::process::Command;
use std::collections::HashMap;

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

struct ScopeData{
    name : String,
}

pub struct Compiler{
    scope_to_data  : HashMap<String, ScopeData>,
    tmp_label_count: u32,
}

impl Compiler{

    pub fn new() -> Compiler{
        Compiler {scope_to_data : HashMap::new(), tmp_label_count: 0}
    }


    fn code_gen(&mut self, node : &serde_json::Value, scope : String, code: &mut Vec<String>){

    }

    fn _compile(&mut self, path_to_c_source : &str) -> Vec<String>{
        let mut code: Vec<String> = Vec::new();
        let ast = get_ast_json(path_to_c_source);
        
        // assuming only main func
        let main_func = &ast["ext"][0];
        let main_decl = &main_func["decl"];
        assert_eq!(main_decl["name"], serde_json::Value::String("main".to_string()));
        assert_eq!(main_decl["type"]["type"]["type"]["names"][0], serde_json::Value::String("int".to_string()));
        self.code_gen(&main_func, "".to_string(), &mut code);

        code
    }   


    pub fn compile(path_to_c_source : &str) -> Vec<String>{
        let mut instance = Compiler::new();
        instance._compile(path_to_c_source)
    }

}