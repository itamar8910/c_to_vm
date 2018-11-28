extern crate serde_json;

use std::process::Command;
use std::collections::HashMap;
use ::cpu::instructions::{Register};

const PATH_TO_PY_EXEC : &str = "src/operating_system/compiler/parser/venv/bin/python";
const PATH_TO_PARSER : &str = "src/operating_system/compiler/parser/to_ast_json.py";

// typedef ast Node = JSON value
use self::serde_json::Value as Node;
fn node_as_string(node: &Node) -> String{
    // helper function because we only has node.as_str()
    String::from(node.as_str().unwrap())
}

pub fn get_ast_json(path_to_c_source: &str) -> Node{
    let output = Command::new(PATH_TO_PY_EXEC)
                        .arg(PATH_TO_PARSER)
                        .arg(path_to_c_source)
                        .output()
                        .expect("Failed to execute c parser");

    let json_str = String::from_utf8(output.stdout).expect("Error decoding ast json bytes");

    serde_json::from_str(&json_str).expect("parser output is not JSON serializable")
}

struct VariableData{
    name: String,
    varType: String,
    offset: u32,
}

struct FuncData{
    scopeData: ScopeData,
    regsUsed: Vec<Register>,
    returnType: String,
}
struct ScopeData{
    name : String,
    parentScope: String,
    variables: HashMap<String, VariableData>,
}

// a scope-like object, i.e either a function or a regular scope
enum ScopeLike{
    Func(FuncData),
    Scope(ScopeData),
}

pub struct Compiler{
    scope_to_data  : HashMap<String, ScopeLike>,
    tmp_label_count: u32,
}

impl Compiler{

    pub fn new() -> Compiler{
        Compiler {scope_to_data : HashMap::new(), tmp_label_count: 0}
    }

    fn node_type(node: &Node) -> String{
        node_as_string(&node["type"])
    }

    // generates code, inserts generated code into the 'code' parameter
    // we want to get code as a paramter rather that having it as a member of Compiler,
    // so we can post-process the code generated for a specific object.
    // an example for usefulness of this is knowing which registers we need to save in a function.
    fn code_gen(&mut self, node : &Node, scope : &String, code: &mut Vec<String>){
        
    }

    fn get_var_type_and_size(&self, node: &Node) -> (String, u32){
        (String::from("int"), 1) // TODO generalize
    }

    fn regiser_function(&mut self, node : &Node, parentScope: &String){

        // collect variables
        let mut next_var_offset = 0;
        let mut variables = HashMap::new();
        let block_items = node["block_items"].as_array().unwrap();
        for item in block_items.iter(){
            if Compiler::node_type(item) == "Decl"{
                let var_name = node_as_string(&item["name"]);
                let (var_type, var_size) = self.get_var_type_and_size(node);
                variables.insert(var_name.clone(), VariableData{
                    name: var_name.clone(),
                    varType: var_type,
                    offset: next_var_offset,
                });
                next_var_offset += var_size;
            }
        }

        let funcName = node_as_string(&node["decl"]["name"]);
        let regsUsed = Vec::new();
        let funcRetType = String::from("int");
        let scopeData = ScopeData{
                name: funcName.clone(),
                parentScope: parentScope.clone(),
                variables: variables,
            };
        self.scope_to_data.insert(funcName, ScopeLike::Func(
            FuncData{
                scopeData: scopeData,
                regsUsed: regsUsed,
                returnType: funcRetType,
        }));
    }

    fn _compile(&mut self, path_to_c_source : &str) -> Vec<String>{
        let mut code: Vec<String> = Vec::new();
        let ast = get_ast_json(path_to_c_source);
        
        // assuming only main func
        let main_func = &ast["ext"][0];
        let main_decl = &main_func["decl"];
        assert_eq!(main_decl["name"], Node::String("main".to_string()));
        assert_eq!(main_decl["type"]["type"]["type"]["names"][0], Node::String("int".to_string()));

        self.code_gen(&main_func, &"".to_string(), &mut code);

        code
    }   


    pub fn compile(path_to_c_source : &str) -> Vec<String>{
        let mut instance = Compiler::new();
        instance._compile(path_to_c_source)
    }

}