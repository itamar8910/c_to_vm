extern crate serde_json;
mod AST;

use std::collections::HashMap;
use ::cpu::instructions::{Register};
use self::AST::*;


// typedef ast Node = JSON value
use self::serde_json::Value as Node;
fn node_as_string(node: &Node) -> String{
    // helper function because we only has node.as_str()
    String::from(node.as_str().unwrap())
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

    fn right_gen(&mut self, node: &Expression, scope : &String, code: &mut Vec<String>){
        match node{
            Expression::Constant(c) => {
                let const_val = &c.val;
                code.push(format!("MOV R1 {}", const_val));
            },
            Expression::BinaryOp(op) => {
               self.right_gen(&op.left, &scope, code); 
               code.push("PUSH R1".to_string());  // save left result on stack
               self.right_gen(&op.right, &scope, code); 
               code.push("POP R2".to_string());
               if let Some(opname) = op.opType.to_op(){
                code.push(format!("{} R1 R2 R1", opname));
               }else{
                   // deal with blooean ops
                    match op.opType{

                        BinaryOpType::EQ => {
                            code.push("TSTE R1 R2".to_string());
                            code.push("MOV R1 ZR".to_string());
                        }

                        BinaryOpType::NEQ => {
                            code.push("TSTN R1 R2".to_string());
                            code.push("MOV R1 ZR".to_string());
                        }

                        BinaryOpType::LOGICAL_AND => {
                            code.push("TSTN R1 0".to_string());
                            code.push("MOV R1 ZR".to_string());
                            code.push("TSTN R2 0".to_string());
                            code.push("AND R1 R1 ZR".to_string());
                        }

                        BinaryOpType::LOGICAL_OR => {
                            code.push("TSTN R1 0".to_string());
                            code.push("MOV R1 ZR".to_string());
                            code.push("TSTN R2 0".to_string());
                            code.push("OR R1 R1 ZR".to_string());
                        }

                        BinaryOpType::LT => {
                            code.push("TSTL R2 R1".to_string());
                            code.push("MOV R1 ZR".to_string());
                        }

                        BinaryOpType::LTEQ => {
                            code.push("TSTG R2 R1".to_string());
                            code.push("TSTN ZR 1".to_string());
                            code.push("MOV R1 ZR".to_string());
                        }

                        BinaryOpType::GT => {
                            code.push("TSTG R2 R1".to_string());
                            code.push("MOV R1 ZR".to_string());
                        }

                        BinaryOpType::GTEQ => {
                            code.push("TSTL R2 R1".to_string());
                            code.push("TSTN ZR 1".to_string());
                            code.push("MOV R1 ZR".to_string());
                        },
                        _ => {
                            panic!("invalid boolean binary op");
                        }
                    }

               }
            },
            Expression::UnaryOp(op) => {
                match op.opType{
                    UnaryOpType::NEG => {
                        self.right_gen(&op.expr, &scope, code);
                        code.push("NEG R1".to_string());
                    },
                    UnaryOpType::NOT => {
                        self.right_gen(&op.expr, &scope, code);
                        code.push("TSTE R1 0".to_string());
                        code.push("MOV R1 ZR".to_string());
                    }
                }
            },
            Expression::ID(id) => {
                let var_name = &id.name;
                self.load_addr_of(&var_name, &scope, code);
                code.push("LOAD R1 R1".to_string());
            },
            Expression::Assignment(ass) => {
                self.gen_assignment_code(ass, &scope, code);
            }
        }
    }

    // generates code for assignment
    // at the end of the generated code, value of assignment is in R1
    fn gen_assignment_code(&mut self, ass : &Assignment, scope : &String, code: &mut Vec<String>){
            self.left_gen(&ass.lvalue, &scope, code);
            code.push("PUSH R1".to_string());
            self.right_gen(&ass.rvalue, &scope, code);
            code.push("POP R2".to_string());
            // now R1 holds rvalue, R2 holds lvalue
            if let Some(bop) = &ass.op.op{
                // if assignment is e.g +=, -=
                code.push("PUSH R2".to_string());
                code.push("LOAD R2 R2 R2".to_string());
                code.push(format!("{} R1 R2 R1", bop.to_op().unwrap()));
                code.push("POP R2".to_string());

            }
            code.push("STR R2 R1".to_string());

    }

    fn load_addr_of(&mut self, var_name: &String, scope: &String, code: &mut Vec<String>){
        // TODO: support non-function scopes
        // TODO: support looking form variables defined in ancestor scopes
        if let Some(scope) = self.scope_to_data.get(scope){
            if let ScopeLike::Func(func) = scope{
                let var = func.scopeData.variables.get(var_name).unwrap();
                let offset = var.offset;
                let var_offset_from_bp = -((1 + func.regsUsed.len() as u32 + offset) as i32);
                code.push(format!("ADD R1 BP {}", var_offset_from_bp));
            }else{
                panic!("currently only function scopes are supported");
            }
        }else{
            panic!("Invalid scope");
        }
    }


    // after executing the generated code, evaluate daddress is stored in R1
    fn left_gen(&mut self, node: &Expression, scope : &String, code: &mut Vec<String>){
        match node{
            Expression::ID(id) => {
                let var_name = &id.name;
                self.load_addr_of(&var_name, &scope, code);
            },
            _ => {
                panic!("not yet supported as an lvalue")
            }
        }
    }

    // generates code, inserts generated code into the 'code' parameter
    // we want to get code as a paramter rather that having it as a member of Compiler,
    // so we can post-process the code generated for a specific object.
    // an example for usefulness of this is knowing which registers we need to save in a function.
    fn code_gen(&mut self, node: AST::AstNode, scope : &String, code: &mut Vec<String>){
        match node{
            AstNode::FuncDef(func_def) => {

                let funcName = &func_def.decl.name;
                self.regiser_function(func_def, scope);
                { // NLL workaround
                    let func_data = self.get_func_data(&funcName);
                    println!("regs used:{:?}", func_data.regsUsed);
                    // save registers
                    for reg in func_data.regsUsed.iter(){
                        println!("saving reg:{}", reg);
                        code.push(format!("PUSH {}", reg.to_str()));
                    }
                    // make space on stack for local variables
                    for (_, var_data) in &func_data.scopeData.variables{
                        for _ in 0..var_data.size{
                            // R1 contains "garbage", but we're just making space
                            code.push(String::from("PUSH R1"));
                        }
                    }
                }

                self.code_gen(AstNode::Compound(&func_def.body), &funcName, code);

                code.push(format!("_{}_END:", funcName));

                // restore registers
                let func_data = self.get_func_data(&funcName);

                // save registers
                for reg in func_data.regsUsed.iter().rev(){
                    code.push(format!("POP {}", reg.to_str()));
                }
                code.push("RET".to_string());

            },
            AstNode::Compound(compound)  => {
                for item in compound.items.iter(){
                    self.code_gen(AstNode::Statement(&item), &scope, code);
                }
            },
            AstNode::Statement(statement) => {
                match statement{
                    Statement::Return(ret) => {
                        self.right_gen(&ret.expr, &scope, code);
                        code.push("ADD R2 BP 2".to_string());
                        code.push("STR R2 R1 ".to_string());
                        code.push(format!("JUMP _{}_END", scope));
                    },
                    Statement::Decl(decl) => {
                        if let Some(expr) = &decl.init{
                            // if decleration is also initialization
                            self.load_addr_of(&decl.name, &scope, code);
                            code.push("PUSH R1".to_string());
                            self.right_gen(&expr, &scope, code);
                            code.push("POP R2".to_string());
                            code.push("STR R2 R1".to_string());
                        }
                    },
                    Statement::Assignment(ass) => {
                        self.gen_assignment_code(ass, &scope, code);
                    }
                }
            }
            _ => {
                panic!("Unkown node type");
            }
        }
    }

    // fn get_var_type_and_size(&self, node: &Node) -> (String, u32){
    //     (String::from("int"), 1) // TODO generalize
    // }

    fn get_type_size(&self, _type: &String) -> u32{
        1 // TODO: generalize
    }

    // registers function's data, returns its name
    fn regiser_function(&mut self, func_def: &FuncDef, parentScope: &String){
        let func_name = &func_def.decl.name; 
        // collect variables
        let mut next_var_offset = 0;
        let mut variables = HashMap::new();
        let block_items = &func_def.body.items;
        for item in block_items.iter(){
            if let Statement::Decl(decl)  = item{
                let var_name = &decl.name;
                let var_type = &decl._type;
                let var_size = self.get_type_size(&var_type);
                variables.insert(var_name.clone(), VariableData{
                    name: var_name.clone(),
                    varType: var_type.clone(),
                    offset: next_var_offset,
                    size: var_size,
                });
                next_var_offset += var_size;
            }
        }

        let regsUsed = vec!{Register::R1, Register::R2};
        let funcRetType = String::from("int");
        let scopeData = ScopeData{
                name: func_name.clone(),
                parentScope: parentScope.clone(),
                variables: variables,
            };
        let funcData = FuncData{
                scopeData: scopeData,
                regsUsed: regsUsed,
                returnType: funcRetType,
        };
        self.scope_to_data.insert(func_name.clone(), ScopeLike::Func(funcData));
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
        let ast = AST::get_ast(path_to_c_source);
        // println!("{}", ast);
        
        // assuming only main func
        let External::FuncDef(main_func) = &ast.externals[0];
        let main_decl = &main_func.decl;
        assert_eq!(main_decl.name, "main".to_string());
        assert_eq!(main_decl.retType, "int".to_string());

        self.code_gen(AstNode::FuncDef(&main_func), &"".to_string(), &mut code);

        code

    }   


    pub fn compile(path_to_c_source : &str) -> String{
        let mut instance = Compiler::new();
        let instructions = instance._compile(path_to_c_source);
        instructions.join("\n")
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