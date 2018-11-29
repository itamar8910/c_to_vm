extern crate serde_json;
mod AST;

use std::collections::HashMap;
use ::cpu::instructions::{Register};


// typedef ast Node = JSON value
use self::serde_json::Value as Node;
fn node_as_string(node: &Node) -> String{
    // helper function because we only has node.as_str()
    String::from(node.as_str().unwrap())
}

pub fn get_ast_json(path_to_c_source: &str) -> Node{
    // //TODO: refactor to using our strong-typed AST module
    // let output = Command::new(PATH_TO_PY_EXEC)
    //                     .arg(PATH_TO_PARSER)
    //                     .arg(path_to_c_source)
    //                     .output()
    //                     .expect("Failed to execute c parser");

    // let json_str = String::from_utf8(output.stdout).expect("Error decoding ast json bytes");

    // serde_json::from_str(&json_str).expect("parser output is not JSON serializable")
    serde_json::from_str(&"{}").unwrap()  // TODO: switch to AST module
}

struct VariableData{
    name: String,
    varType: String,
    offset: u32,
    size: u32,
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
        node_as_string(&node["_nodetype"])
    }

    fn get_size_of_type(&self, varType: &String) -> u32{
        1  // TODO: generalize
    }

    // generates code, inserts generated code into the 'code' parameter
    // we want to get code as a paramter rather that having it as a member of Compiler,
    // so we can post-process the code generated for a specific object.
    // an example for usefulness of this is knowing which registers we need to save in a function.
    fn code_gen(&mut self, node : &Node, scope : &String, code: &mut Vec<String>){
        match Compiler::node_type(&node).as_ref(){
            "FuncDef" => {
                let funcName = node_as_string(&node["decl"]["name"]);
                self.regiser_function(&node["body"], scope, &funcName);
                { // NLL workaround
                    let func_data = self.get_func_data(&funcName);

                    // save registers
                    for reg in func_data.regsUsed.iter(){
                        code.push(format!("PUSH {}", reg.to_str()));
                    }
                    // make space on stack for local variables
                    for (_, var_data) in &func_data.scopeData.variables{
                        for _ in 0..var_data.size{
                            code.push(String::from("PUSH 0"));
                        }
                    }
                }

                self.code_gen(&node["body"], &funcName, code);

                code.push(format!("_{}_END:", funcName));

                // restore registers
                let func_data = self.get_func_data(&funcName);

                // save registers
                for reg in func_data.regsUsed.iter().rev(){
                    code.push(format!("POP {}", reg.to_str()));
                }
                code.push("RET".to_string());

            },
            "Compound" => {
                // for item in node["block_items"].as_array().unwrap().iter(){
                //     self.code_gen(item, &scope, code);
                // }
            }
            _ => {
                panic!("Unkown node type: {}", Compiler::node_type(&node));
            }
        }
    }

    fn get_var_type_and_size(&self, node: &Node) -> (String, u32){
        (String::from("int"), 1) // TODO generalize
    }

    // registers function's data, returns its name
    fn regiser_function(&mut self, node : &Node, parentScope: &String, funcName: &String){

        // collect variables
        let mut next_var_offset = 0;
        let mut variables = HashMap::new();
        println!("{}", node);
        let block_items = node["block_items"].as_array().unwrap();
        for item in block_items.iter(){
            if Compiler::node_type(item) == "Decl"{
                let var_name = node_as_string(&item["name"]);
                let (var_type, var_size) = self.get_var_type_and_size(node);
                variables.insert(var_name.clone(), VariableData{
                    name: var_name.clone(),
                    varType: var_type.clone(),
                    offset: next_var_offset,
                    size: self.get_size_of_type(&var_type),
                });
                next_var_offset += var_size;
            }
        }

        let regsUsed = vec!{Register::R1, Register::R2};
        let funcRetType = String::from("int");
        let scopeData = ScopeData{
                name: funcName.clone(),
                parentScope: parentScope.clone(),
                variables: variables,
            };
        let funcData = FuncData{
                scopeData: scopeData,
                regsUsed: regsUsed,
                returnType: funcRetType,
        };
        self.scope_to_data.insert(funcName.clone(), ScopeLike::Func(funcData));

    }

    fn get_func_data(&self, func_name : &String) -> &FuncData{
        if let Some(ScopeLike::Func(fd)) = self.scope_to_data.get(&func_name.to_string()){
            &fd
        }else{
            panic!();
        }
    }

    fn _compile(&mut self, path_to_c_source : &str) -> Vec<String>{
        let mut code: Vec<String> = Vec::new();
        let ast = get_ast_json(path_to_c_source);
        println!("{}", ast);
        
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



// #[cfg(test)]
// mod tests{
//     use super::*;
//     #[test]
//     fn empty_main(){
//         let code = Compiler::compile("src/operating_system/compiler/test_data/const_expressions/inputs/1.c");
//         print!("{:?}", code);
//     }
// }