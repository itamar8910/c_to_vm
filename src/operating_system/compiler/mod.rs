extern crate serde_json;
mod AST;

use self::AST::*;
use cpu::instructions::Register;
use std::collections::HashMap;

// typedef ast Node = JSON value
use self::serde_json::Value as Node;

struct VariableData {
    name: String,
    varType: String,
    offset: u32,
    size: u32,
    declared: bool,
}

struct FuncData {
    scope_data: ScopeData,
    regs_used: Vec<Register>,
    returnType: String,
}

struct ScopeData {
    name: String,
    parent_scope: String,
    variables: HashMap<String, VariableData>,
}

// a scope-like object, i.e either a function or a regular scope
enum ScopeLike {
    Func(FuncData),
    Scope(ScopeData),
}

pub struct Compiler {
    scope_to_data: HashMap<String, ScopeLike>,
    tmp_label_count: u32,
}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {
            scope_to_data: HashMap::new(),
            tmp_label_count: 0,
        }
    }

    fn right_gen(&mut self, node: &Expression, scope: &String, code: &mut Vec<String>) {
        match node {
            Expression::Constant(c) => {
                let const_val = &c.val;
                code.push(format!("MOV R1 {}", const_val));
            }
            Expression::BinaryOp(op) => {
                self.right_gen(&op.left, &scope, code);
                code.push("PUSH R1".to_string()); // save left result on stack
                self.right_gen(&op.right, &scope, code);
                code.push("POP R2".to_string());
                if let Some(opname) = op.op_type.to_op() {
                    code.push(format!("{} R1 R2 R1", opname));
                } else {
                    // deal with blooean ops
                    match op.op_type {
                        BinaryopType::EQ => {
                            code.push("TSTE R1 R2".to_string());
                            code.push("MOV R1 ZR".to_string());
                        }

                        BinaryopType::NEQ => {
                            code.push("TSTN R1 R2".to_string());
                            code.push("MOV R1 ZR".to_string());
                        }

                        BinaryopType::LogicalAnd => {
                            code.push("TSTN R1 0".to_string());
                            code.push("MOV R1 ZR".to_string());
                            code.push("TSTN R2 0".to_string());
                            code.push("AND R1 R1 ZR".to_string());
                        }

                        BinaryopType::LogicalOr => {
                            code.push("TSTN R1 0".to_string());
                            code.push("MOV R1 ZR".to_string());
                            code.push("TSTN R2 0".to_string());
                            code.push("OR R1 R1 ZR".to_string());
                        }

                        BinaryopType::LT => {
                            code.push("TSTL R2 R1".to_string());
                            code.push("MOV R1 ZR".to_string());
                        }

                        BinaryopType::LTEQ => {
                            code.push("TSTG R2 R1".to_string());
                            code.push("TSTN ZR 1".to_string());
                            code.push("MOV R1 ZR".to_string());
                        }

                        BinaryopType::GT => {
                            code.push("TSTG R2 R1".to_string());
                            code.push("MOV R1 ZR".to_string());
                        }

                        BinaryopType::GTEQ => {
                            code.push("TSTL R2 R1".to_string());
                            code.push("TSTN ZR 1".to_string());
                            code.push("MOV R1 ZR".to_string());
                        }
                        _ => {
                            panic!("invalid boolean binary op");
                        }
                    }
                }
            }
            Expression::UnaryOp(op) => {
                match op.op_type {
                    UnaryopType::NEG => {
                        self.right_gen(&op.expr, &scope, code);
                        code.push("NEG R1".to_string());
                    }
                    UnaryopType::NOT => {
                        self.right_gen(&op.expr, &scope, code);
                        code.push("TSTE R1 0".to_string());
                        code.push("MOV R1 ZR".to_string());
                    }
                    UnaryopType::PPX | UnaryopType::MMX => {
                        self.left_gen(&op.expr, &scope, code);
                        code.push("LOAD R2 R1".to_string());
                        // let add_or_sub = match op.op_type{
                        //     UnaryopType::PPX()
                        // }
                        code.push(format!(
                            "{} R2 R2 1",
                            if op.op_type == UnaryopType::PPX {
                                "ADD"
                            } else {
                                "SUB"
                            }
                        ));
                        code.push("STR R1 R2".to_string());
                        code.push("MOV R1 R2".to_string());
                    }
                    UnaryopType::XPP | UnaryopType::XMM => {
                        self.left_gen(&op.expr, &scope, code);
                        code.push("LOAD R2 R1".to_string());
                        code.push("PUSH R2".to_string());
                        code.push(format!(
                            "{} R2 R2 1",
                            if op.op_type == UnaryopType::XPP {
                                "ADD"
                            } else {
                                "SUB"
                            }
                        ));
                        code.push("STR R1 R2".to_string());
                        code.push("POP R1".to_string());
                    }
                }
            }
            Expression::ID(id) => {
                let var_name = &id.name;
                self.load_addr_of(&var_name, &scope, code);
                code.push("LOAD R1 R1".to_string());
            }
            Expression::Assignment(ass) => {
                self.gen_assignment_code(ass, &scope, code);
            }
            Expression::TernaryOp(top) => {
                let neg_label = format!("TERNARY_{}_NO", self.tmp_label_count);
                let ternary_end_label = format!("TERNARY_{}_YES", self.tmp_label_count);
                self.tmp_label_count += 1;
                self.right_gen(&top.cond, &scope, code);
                code.push("TSTN R1 0".to_string());
                code.push(format!("FJMP {}", neg_label));
                self.right_gen(&*top.iftrue, &scope, code);
                code.push(format!("JUMP {}", ternary_end_label));
                code.push(format!("{}:", neg_label));
                self.right_gen(&*top.iffalse, &scope, code);
                code.push(format!("{}:", ternary_end_label));
            }
        }
    }

    // generates code for assignment
    // at the end of the generated code, value of assignment is in R1
    fn gen_assignment_code(&mut self, ass: &Assignment, scope: &String, code: &mut Vec<String>) {
        self.left_gen(&ass.lvalue, &scope, code);
        code.push("PUSH R1".to_string());
        self.right_gen(&ass.rvalue, &scope, code);
        code.push("POP R2".to_string());
        // now R1 holds rvalue, R2 holds lvalue
        if let Some(bop) = &ass.op.op {
            // if assignment is e.g +=, -=
            code.push("PUSH R2".to_string());
            code.push("LOAD R2 R2".to_string());
            code.push(format!("{} R1 R2 R1", bop.to_op().unwrap()));
            code.push("POP R2".to_string());
        }
        code.push("STR R2 R1".to_string());
    }

    fn load_addr_of(&mut self, var_name: &String, scope: &String, code: &mut Vec<String>) {
        // TODO: support non-function scopes
        // TODO: support looking form variables defined in ancestor scopes
        if let Some(scope) = self.scope_to_data.get(scope) {
            if let ScopeLike::Func(func) = scope {
                let var = func.scope_data.variables.get(var_name).expect(&format!("variable:{} does not exist in scope:{}", var_name, func.scope_data.name));
                let offset = var.offset;
                let var_offset_from_bp = -((1 + func.regs_used.len() as u32 + offset) as i32);
                code.push(format!("ADD R1 BP {}", var_offset_from_bp));
            } else {
                panic!("currently only function scopes are supported");
            }
        } else {
            panic!("Invalid scope");
        }
    }

    // after executing the generated code, evaluate daddress is stored in R1
    fn left_gen(&mut self, node: &Expression, scope: &String, code: &mut Vec<String>) {
        match node {
            Expression::ID(id) => {
                let var_name = &id.name;
                self.load_addr_of(&var_name, &scope, code);
            }
            _ => panic!("not yet supported as an lvalue"),
        }
    }

    // generates code, inserts generated code into the 'code' parameter
    // we want to get code as a paramter rather that having it as a member of Compiler,
    // so we can post-process the code generated for a specific object.
    // an example for usefulness of this is knowing which registers we need to save in a function.
    fn code_gen(&mut self, node: AST::AstNode, scope: &String, code: &mut Vec<String>) {
        match node {
            AstNode::FuncDef(func_def) => {
                let func_name = &func_def.decl.name;
                self.regiser_function(func_def, scope);
                {
                    // NLL workaround
                    let func_data = self.get_func_data(&func_name);
                    println!("regs used:{:?}", func_data.regs_used);
                    // save registers
                    for reg in func_data.regs_used.iter() {
                        println!("saving reg:{}", reg);
                        code.push(format!("PUSH {}", reg.to_str()));
                    }
                    // make space on stack for local variables
                    for (_, var_data) in &func_data.scope_data.variables {
                        for _ in 0..var_data.size {
                            // R1 contains "garbage", but we're just making space
                            code.push(String::from("PUSH R1"));
                        }
                    }
                }

                self.code_gen(AstNode::Compound(&func_def.body), &func_name, code);

                code.push(format!("_{}_END:", func_name));

                // restore registers
                let func_data = self.get_func_data(&func_name);
                // dealocate stack space of local variables
                for (_, var_data) in &func_data.scope_data.variables {
                    for _ in 0..var_data.size {
                        // R1 contains "garbage", but we're just making space
                        code.push(String::from("POP R2"));
                    }
                }

                // save registers
                for reg in func_data.regs_used.iter().rev() {
                    code.push(format!("POP {}", reg.to_str()));
                }
                code.push("RET".to_string());
            }
            AstNode::Compound(compound) => {
                for item in compound.items.iter() {
                    self.code_gen(AstNode::Statement(&item), &scope, code);
                }
            }
            AstNode::Statement(statement) => {
                match statement {
                    Statement::Return(ret) => {
                        self.right_gen(&ret.expr, &scope, code);
                        code.push("ADD R2 BP 2".to_string());
                        code.push("STR R2 R1 ".to_string());
                        code.push(format!("JUMP _{}_END", scope));
                    }
                    Statement::Decl(decl) => {
                        // self.update_var_declared(&decl.name);
                        if let Some(expr) = &decl.init {
                            // if decleration is also initialization
                            self.load_addr_of(&decl.name, &scope, code);
                            code.push("PUSH R1".to_string());
                            self.right_gen(&expr, &scope, code);
                            code.push("POP R2".to_string());
                            code.push("STR R2 R1".to_string());
                        }
                    }
                    Statement::Assignment(ass) => {
                        self.gen_assignment_code(ass, &scope, code);
                    }
                    Statement::Expression(exp) => {
                        self.right_gen(&exp, &scope, code);
                    }
                    Statement::If(if_stmt) => {
                        let else_label = format!("IF_{}_ELSE", self.tmp_label_count);
                        let if_end_label = format!("IF_{}_END", self.tmp_label_count);
                        self.tmp_label_count += 1;
                        // TODO: create a new scope for the if statement
                        self.right_gen(&if_stmt.cond, &scope, code);
                        code.push("TSTN R1 0".to_string());
                        code.push(format!("FJMP {}", else_label));
                        self.code_gen(AstNode::Compound(&*if_stmt.iftrue), &scope, code);
                        code.push(format!("JUMP {}", if_end_label));
                        code.push(format!("{}:", else_label));
                        match &if_stmt.iffalse.as_ref() {
                            Some(ref iffalse) => {
                                self.code_gen(AstNode::Compound(&*(*iffalse)), &scope, code);
                            }
                            None => {}
                        }
                        code.push(format!("{}:", if_end_label));
                    }
                }
            }
            _ => {
                panic!("Unkown node type");
            }
        }
    }

    fn find_variable(&mut self, var_name: &String, scope: &String) -> Option<&mut VariableData>{
        let mut cur_scope_name = scope;
        loop{
            let cur_scope = self.scope_to_data.get_mut(cur_scope_name.clone().as_str()).expect("scope doesn't exist");
            let mut scope_data = match cur_scope{
                ScopeLike::Func(func_data) => & func_data.scope_data,
                ScopeLike::Scope(s) => {
                    s
                   },
            };
            if let Some(x) = scope_data.variables.get_mut(var_name.as_str()){
                return Some(x);
            }else{
                cur_scope_name = &(scope_data.parent_scope);
                if cur_scope_name == ""{
                    return None
                }
            }
        }
    }

    // fn update_var_declared(&mut self, var_name: &String){
    //     let x = self.scope_to_data.get(&var_name).expect("Variable doesn'te exist")
    // }

    // fn get_var_type_and_size(&self, node: &Node) -> (String, u32){
    //     (String::from("int"), 1) // TODO generalize
    // }

    fn get_type_size(&self, _type: &String) -> u32 {
        1 // TODO: generalize
    }

    // registers function's data, returns its name
    fn regiser_function(&mut self, func_def: &FuncDef, parent_scope: &String) {
        let func_name = &func_def.decl.name;
        // collect variables
        let mut next_var_offset = 0;
        let mut variables = HashMap::new();
        let block_items = &func_def.body.items;
        for item in block_items.iter() {
            if let Statement::Decl(decl) = item {
                let var_name = &decl.name;
                let var_type = &decl._type;
                let var_size = self.get_type_size(&var_type);
                variables.insert(
                    var_name.clone(),
                    VariableData {
                        name: var_name.clone(),
                        varType: var_type.clone(),
                        offset: next_var_offset,
                        size: var_size,
                        declared: false,
                    },
                );
                next_var_offset += var_size;
            }
        }

        let regs_used = vec![Register::R1, Register::R2];
        let funcret_type = String::from("int");
        let scope_data = ScopeData {
            name: func_name.clone(),
            parent_scope: parent_scope.clone(),
            variables: variables,
        };
        let func_data = FuncData {
            scope_data: scope_data,
            regs_used: regs_used,
            returnType: funcret_type,
        };
        self.scope_to_data
            .insert(func_name.clone(), ScopeLike::Func(func_data));
    }

    fn get_func_data(&self, func_name: &String) -> &FuncData {
        if let Some(ScopeLike::Func(fd)) = self.scope_to_data.get(&func_name.to_string()) {
            &fd
        } else {
            panic!();
        }
    }

    fn _compile(&mut self, path_to_c_source: &str) -> Vec<String> {
        let mut code: Vec<String> = Vec::new();
        let ast = AST::get_ast(path_to_c_source);
        // println!("{}", ast);

        // assuming only main func
        let External::FuncDef(main_func) = &ast.externals[0];
        let main_decl = &main_func.decl;
        assert_eq!(main_decl.name, "main".to_string());
        assert_eq!(main_decl.ret_type, "int".to_string());

        self.code_gen(AstNode::FuncDef(&main_func), &"".to_string(), &mut code);

        code
    }

    pub fn compile(path_to_c_source: &str) -> String {
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
