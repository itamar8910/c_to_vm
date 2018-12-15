
use std::fs::File;
use std::io::prelude::*;

extern crate serde_json;

extern crate tempfile;
use tempfile::NamedTempFile;
use std::io::{Write};

extern crate linked_hash_map;
use linked_hash_map::LinkedHashMap;

mod AST;
mod preprocessor;

use self::AST::*;
use crate::cpu::instructions::Register;
use std::collections::HashMap;
use std::collections::HashSet;

// typedef ast Node = JSON value
use self::serde_json::Value as Node;

#[derive(Debug)]
enum LocalOrArg{
    Local,
    Arg,
}


#[derive(Debug)]
enum VariableType {
    Regular {_type: Type}, // including structs
    Array {_type: Type, dimentions: Vec<u32>},
}

impl VariableType{
    fn from(decl: &Decl) -> VariableType{
        match decl{
            Decl::VarDecl(var_decl) => VariableType::Regular{
                _type: var_decl._type.clone(),
            },
            Decl::ArrayDecl(arr_decl) => VariableType::Array{
                _type: arr_decl._type.clone(),
                dimentions: arr_decl.dimentions.clone(),
            },
        }
    }
}

#[derive(Debug)]
struct VariableData {
    name: String,
    local_or_arg: LocalOrArg,
    var_type: VariableType,
    offset: u32,
    size: u32,
}

impl VariableData{
}

#[derive(Debug)]
struct FuncBodyData {
    name: String,
    regs_used: Vec<Register>,
    local_vars_size: u32,
}

// this is the data that we get once we declare a function
#[derive(Debug)]
struct FuncDeclData{
    args_types : Vec<VariableType>,
    return_type: Type,
}

struct FuncData{
    decl_data: FuncDeclData,
    body_data: Option<FuncBodyData>,
}

#[derive(Debug)]
struct ScopeData {
    name: String,
    parent_scope: String,
    parent_func: String,
    variables: HashMap<String, VariableData>,
    declared_variables: HashSet<String>,
    break_label: Option<String>,
    continue_label: Option<String>,
}


#[derive(Debug)]
pub struct StructData{
    name: String,
    size: u32,
    items: LinkedHashMap<String, VariableData>,
}

pub struct Compiler {
    scope_to_data: HashMap<String, ScopeData>,
    func_to_data: HashMap<String, FuncData>,
    struct_to_data: HashMap<String, StructData>,
    tmp_label_count: u32,
}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {
            scope_to_data: HashMap::new(),
            func_to_data: HashMap::new(),
            struct_to_data: HashMap::new(),
            tmp_label_count: 0,
        }
    }

    fn get_scope_data(&self, scope: &String) -> Option<& ScopeData>{
        self.scope_to_data.get(scope)
    }

    fn get_scope_data_mut(&mut self, scope: &String) -> Option<&mut ScopeData>{
        self.scope_to_data.get_mut(scope)
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
                    },
                    UnaryopType::REF => {
                        self.left_gen(&op.expr, scope, code);
                    },
                    UnaryopType::DEREF => {
                        self.right_gen(&op.expr, scope, code);
                        code.push("LOAD R1 R1".to_string());
                    }
                }
            }
            Expression::ID(id) => {
                let var_name = &id.name;
                self.codegen_load_addr_of_var(&var_name, &scope, code);
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
            },
            Expression::FuncCall(func_call) => {
                let func_data = self.get_func_data(&func_call.name).unwrap();
                let rettype = func_data.decl_data.return_type.clone();
                // push args
                for arg in func_call.args.iter().rev(){
                    self.right_gen(&*arg, scope, code);
                    code.push("PUSH R1".to_string());
                }
                // push space for func retval
                for _ in 0..self.get_type_size(&rettype){
                    code.push("PUSH ZR".to_string());
                }
                code.push(format!("CALL {}", func_call.name));
                if self.get_type_size(&rettype) > 0{
                    // pop retval to R1
                    code.push("POP R1".to_string());
                }
                // pop args
                for arg in func_call.args.iter().rev(){
                    code.push("POP ZR".to_string());
                }
            },
            Expression::ArrayRef(array_ref) => {
                self.codegen_load_addr_of_array_indexing(array_ref, scope, code);
                code.push("LOAD R1 R1".to_string());
            },
            Expression::StructRef(struct_ref) => {
                self.codegen_load_addr_of_struct_ref(struct_ref, scope, code);
                code.push("LOAD R1 R1".to_string());
            }
        }
    }

    fn get_struct_data_from_type(&self, _t: &Type) -> Option<&StructData> {
        if let Type::Struct(struct_name) = _t {
            Some(self.struct_to_data.get(struct_name)?)
        } else {
            None
        }
    }

    fn codegen_load_addr_of_struct_ref(&mut self, struct_ref: &StructRef, scope: &String, code: &mut Vec<String>){
        let var_name = &struct_ref.name;
        self.codegen_load_addr_of_var(var_name, scope, code);
        let var_data = self.find_variable(var_name, scope).unwrap();
        if let VariableType::Regular{_type: struct_type} = &var_data.var_type{
            if let Type::Struct(struct_type) = struct_type{
                let struct_data = self.struct_to_data.get(struct_type).expect("struct doesn't exist");
                code.push("MOV R2 R1".to_string()); // R2 holds current item addr
                let mut cur_struct = struct_data;
                for (field_i, field) in struct_ref.field_names.iter().enumerate(){
                    let field_vardata = cur_struct.items.get(field).expect(&format!("field {} not found in struct {}", field, &cur_struct.name));

                    code.push(format!("ADD R2 R2 {}", field_vardata.offset));
                    if field_i < struct_ref.field_names.len() - 1{
                        if let VariableType::Regular{_type: t} = &field_vardata.var_type{
                            cur_struct = self.get_struct_data_from_type(t).unwrap();
                        }else{
                            panic!();
                        }
                    }
                }
                code.push("MOV R1 R2".to_string());
            } else {panic!()}
        } else{
            panic!();
        }

    }
    fn codegen_load_addr_of_array_indexing(&mut self, array_ref: &ArrayRef, scope: &String, code: &mut Vec<String>){
        self.codegen_load_addr_of_var(&array_ref.name, scope, code);
        let arr_var = self.find_variable(&array_ref.name, scope).expect("array not found");
        match &arr_var.var_type{
            VariableType::Array{_type, dimentions} => {
                // let mut offset = 0;                        
                code.push("MOV R2 R1".to_string()); // R2 holds current item addr
                let mut cur_dimentions_product = 1;
                let item_size = self.get_type_size(_type);

                // hiding from the borrow checker
                let indices = array_ref.indices.clone();
                let dimentions = dimentions.clone();

                for (idx_expr, dimsize) in indices.iter().zip(dimentions){
                    // offset += idx * cur_dimentions_product * item_size;
                    code.push("PUSH R2".to_string()); // save R2
                    self.right_gen(idx_expr, scope, code);
                    code.push("POP R2".to_string());
                    code.push(format!("MUL R1 R1 {}", cur_dimentions_product));
                    code.push(format!("MUL R1 R1 {}", item_size));
                    code.push("ADD R2 R2 R1".to_string());
                    cur_dimentions_product *= dimsize;
                }
                code.push("MOV R1 R2".to_string());

            },
            _ => panic!(format!("{} is no an array type", &array_ref.name)),

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


    fn codegen_load_addr_of_var(&mut self, var_name: &String, scope: &String, code: &mut Vec<String>) {
        let var_data = self.find_variable(var_name, scope).expect(&format!("Variable {} not found", var_name));
        let scope_data = self.get_scope_data(scope).expect("Scope doesn't exist");
        let func_data = self.get_func_data(& scope_data.parent_func).unwrap();
        let func_body_data = &func_data.body_data.as_ref().expect("Function must be defined");
        let var_offset_from_bp = match var_data.local_or_arg{
            LocalOrArg::Local => -((1 + func_body_data.regs_used.len() as u32 + var_data.offset) as i32),
            LocalOrArg::Arg => {
                let func_retval_size = self.get_type_size(&func_data.decl_data.return_type);
                (2 + func_retval_size + var_data.offset) as i32
            }
        };
        code.push(format!("ADD R1 BP {}", var_offset_from_bp));
    }

    // after executing the generated code, evaluate daddress is stored in R1
    fn left_gen(&mut self, node: &Expression, scope: &String, code: &mut Vec<String>) {
        match node {
            Expression::ID(id) => {
                let var_name = &id.name;
                self.codegen_load_addr_of_var(&var_name, &scope, code);
            }
            Expression::UnaryOp(uop) => {
                match uop.op_type{
                    UnaryopType::DEREF => {
                        self.left_gen(&uop.expr, scope, code);
                        code.push("LOAD R1 R1".to_string());
                    },
                    _ => panic!("only dereference unary op allowed as lvalue")
                }
            },
            Expression::ArrayRef(array_ref) => {
                self.codegen_load_addr_of_array_indexing(array_ref, scope, code);
            },
            Expression::StructRef(struct_ref) => {
                self.codegen_load_addr_of_struct_ref(struct_ref, scope, code);
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
            AstNode::RootAstNode(root_node) => {
                // insert global scope
                self.scope_to_data.insert("_GLOBAL".to_string(), ScopeData {
                    name: "_GLOBAL".to_string(),
                    parent_scope: "_GLOBAL".to_string(),
                    parent_func:  "_GLOBAL".to_string(),
                    variables: HashMap::new(),
                    declared_variables: HashSet::new(),
                    break_label: None,
                    continue_label: None
                });
                code.push("JUMP main".to_string());
                for ext in root_node.externals.iter(){
                    match ext{
                        External::FuncDef(func_def) => {
                            self.code_gen(AstNode::FuncDef(func_def), &"_GLOBAL".to_string(), code);
                        },
                        External::FuncDecl(func_decl) => {
                            self.code_gen(AstNode::FuncDecl(func_decl), &"_GLOBAL".to_string(), code);
                        },
                        External::StructDecl(struct_decl) => {
                            self.register_struct(struct_decl);
                        }
                    }
                }
            },
            AstNode::FuncDecl(func_decl) => {
                let func_name = &func_decl.name;
                if !self.scope_to_data.contains_key(func_name){
                    self.register_func_decl(func_decl);
                }
            }
            AstNode::FuncDef(func_def) => {
                let func_name = &func_def.decl.name;
                code.push(format!("{}:", func_name));
                self.register_func_decl(&func_def.decl);
                self.register_func_body(&func_def.body, &func_def.decl, scope);
                {
                    // NLL workaround
                    let func_data = self.get_func_data(func_name).unwrap();
                    let func_data = &func_data.body_data.as_ref().unwrap();
                    println!("regs used:{:?}", func_data.regs_used);
                    // save registers
                    for reg in func_data.regs_used.iter() {
                        println!("saving reg:{}", reg);
                        code.push(format!("PUSH {}", reg.to_str()));
                    }
                    // make space on stack for local variables
                    let _scope_data = self.get_scope_data(func_name).unwrap();
                    println!("local vars size:{}", func_data.local_vars_size);
                    for _ in 0..func_data.local_vars_size {
                            // ZR contains "garbage", but we're just making space
                            code.push(String::from("PUSH ZR"));
                    }
                }

                self.code_gen(AstNode::Compound(&func_def.body), &func_name, code);

                code.push(format!("_{}_END:", func_name));

                // restore registers
                let func_data = self.get_func_data(&func_name).unwrap();
                let func_data = &func_data.body_data.as_ref().unwrap();
                let _scope_data = self.get_scope_data(func_name).unwrap();
                // dealocate stack space of local variables
                    for _ in 0..func_data.local_vars_size {
                        // ZR contains "garbage", but we're just making space
                        code.push(String::from("POP ZR"));
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
                        code.push(format!("JUMP _{}_END", self.get_scope_data(scope).unwrap().parent_func));
                    }
                    Statement::Decl(decl) => {
                        match decl{
                            Decl::VarDecl(var_decl) => {
                                self.update_var_declared(&var_decl.name, scope);
                                if let Some(expr) = &var_decl.init {
                                    // if decleration is also initialization
                                    self.codegen_load_addr_of_var(&var_decl.name, &scope, code);
                                    code.push("PUSH R1".to_string());
                                    self.right_gen(&expr, &scope, code);
                                    code.push("POP R2".to_string());
                                    code.push("STR R2 R1".to_string());
                                }
                            },
                            Decl::ArrayDecl(arr_decl) => {
                                self.update_var_declared(&arr_decl.name, scope);
                                if let Some(init) = &arr_decl.init{
                                    self.get_arr_init_code(&arr_decl.name, init, scope, code);
                                }
                                                        
                            }
                            _ => panic!("not yet implemented"),
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
                        self.right_gen(&if_stmt.cond, &scope, code);
                        code.push("TSTN R1 0".to_string());
                        code.push(format!("FJMP {}", else_label));
                        self.code_gen(AstNode::Compound(&*if_stmt.iftrue), &if_stmt.iftrue.code_loc, code);
                        code.push(format!("JUMP {}", if_end_label));
                        code.push(format!("{}:", else_label));
                        match &if_stmt.iffalse.as_ref() {
                            Some(ref iffalse) => {
                                self.code_gen(AstNode::Compound(&*(*iffalse)), &iffalse.code_loc, code);
                            }
                            None => {}
                        }
                        code.push(format!("{}:", if_end_label));
                    },
                    Statement::Compound(comp) => {
                        self.code_gen(AstNode::Compound(&comp), &comp.code_loc, code);
                    },
                    Statement::WhileLoop(wl) => {
                        let while_start = format!("WHILE_{}_START", self.tmp_label_count);
                        let while_end = format!("WHILE_{}_END", self.tmp_label_count);
                        self.tmp_label_count += 1;
                        self.update_scope_break_continue_labels(&wl.code_loc, &while_end, &while_start);
                        code.push(format!("{}:", while_start));
                        self.right_gen(&wl.cond, scope, code);
                        code.push("TSTN R1 0".to_string());
                        code.push(format!("FJMP {}", while_end));
                        self.code_gen(AstNode::Compound(&wl.body), &wl.code_loc, code);
                        code.push(format!("JUMP {}", while_start));
                        code.push(format!("{}:", while_end));
                    },
                    Statement::DoWhileLoop(dwl) => {
                        let dowhile_cond = format!("DOWHILE_{}_COND", self.tmp_label_count);
                        let dowhile_body = format!("DOWHILE_{}_BODY", self.tmp_label_count);
                        let dowhile_end = format!("DOWHILE_{}_END", self.tmp_label_count);
                        self.tmp_label_count += 1;
                        self.update_scope_break_continue_labels(&dwl.code_loc, &dowhile_end, &dowhile_cond);
                        code.push(format!("JUMP {}", dowhile_body));
                        code.push(format!("{}:", dowhile_cond));
                        self.right_gen(&dwl.cond, scope, code);
                        code.push("TSTN R1 0".to_string());
                        code.push(format!("FJMP {}", dowhile_end));
                        code.push(format!("{}:", dowhile_body));
                        self.code_gen(AstNode::Compound(&dwl.body), &dwl.code_loc, code);
                        code.push(format!("JUMP {}", dowhile_cond));
                        code.push(format!("{}:", dowhile_end));
                    },
                    Statement::ForLoop(fl) => {
                        let for_cond = format!("FOR_{}_COND", self.tmp_label_count);
                        let for_end = format!("FOR_{}_END", self.tmp_label_count);
                        let for_next = format!("FOR_{}_NEXT", self.tmp_label_count);
                        self.tmp_label_count += 1;
                        self.update_scope_break_continue_labels(&fl.code_loc, &for_end, &for_next);
                        if let Some(init) = &fl.init{
                            self.code_gen(AstNode::Compound(init), &fl.code_loc, code);
                        }
                        code.push(format!("{}:", for_cond));
                        if let Some(cond) = &fl.cond{
                            self.right_gen(cond, &fl.code_loc, code);
                            code.push("TSTN R1 0".to_string());
                            code.push(format!("FJMP {}", for_end));
                        }
                        self.code_gen(AstNode::Compound(&fl.body), &fl.code_loc, code);
                        code.push(format!("{}:", for_next));  // we need the next label even if next part of empty for "continue"
                        if let Some(next) = &fl.next{
                            self.code_gen(AstNode::Compound(next), &fl.code_loc, code);
                        }
                        code.push(format!("JUMP {}", for_cond));
                        code.push(format!("{}:", for_end));
                    },
                    Statement::Break => {
                        let (break_label, _) = self.find_break_continue_labels(scope).unwrap();
                        code.push(format!("JUMP {}", break_label));
                    },
                    Statement::Continue => {
                        let (_, continue_label) = self.find_break_continue_labels(scope).unwrap();
                        code.push(format!("JUMP {}", continue_label));
                    }
                }
            }
            _ => {
                panic!("Unkown node type");
            }
        }
    }

    fn get_arr_init_code(&mut self, arr_name: &String, arr_init: &Vec<Expression>, scope: &String, code: &mut Vec<String>){
        let arr_var = self.find_variable(arr_name, scope).expect("array not found");
        match &arr_var.var_type{
            VariableType::Array{_type, dimentions} => {
                let item_size = self.get_type_size(_type);
                self.codegen_load_addr_of_var(arr_name, scope, code);
                code.push("MOV R2 R1".to_string());
                for expr in arr_init.iter(){
                    code.push("PUSH R2".to_string());
                    self.right_gen(expr, scope, code);
                    code.push("POP R2".to_string());
                    code.push("STR R2 R1".to_string());
                    code.push(format!("ADD R2 R2 {}", item_size));
                }
            },
            _ => panic!(),
        }
    }
    fn find_break_continue_labels(&self, scope: &String) -> Option<(&String, &String)>{
        let mut cur_scope_name = scope;
        loop{
            let scope_data = self.get_scope_data(cur_scope_name).expect(&format!("scope:{} doesn't exist", cur_scope_name));
            if let Some(break_label) = &scope_data.break_label{
                let continue_label = &scope_data.continue_label.as_ref().expect("scope has break label but not continue label");
                return Some((break_label, continue_label))
            }            
            {
                if cur_scope_name == "_GLOBAL"{
                    return None
                }
                cur_scope_name = &(scope_data.parent_scope);
            }
        }
    }
    fn update_scope_break_continue_labels(&mut self, scope: &String, break_label: &String, continue_label: &String){
        let scope_data = self.get_scope_data_mut(scope).expect("scope doesn't exist");
        scope_data.break_label = Some(break_label.clone());
        scope_data.continue_label = Some(continue_label.clone());
    }

    fn find_variable(&self, var_name: &String, scope: &String) -> Option<&VariableData>{
        let mut cur_scope_name = scope;
        loop{
            println!("seraching for var {} inside scope {}", var_name, cur_scope_name);
            let scope_data = self.get_scope_data(cur_scope_name).expect(&format!("scope:{} doesn't exist", cur_scope_name));
            if let Some(x) = scope_data.variables.get(var_name.as_str()){
                if scope_data.declared_variables.contains(var_name){
                    return Some(x);
                }else{
                    println!("found var {} in scope but it isn't declared yet", var_name);
                }
            }
            {
                if cur_scope_name == "_GLOBAL"{
                    return None
                }
                cur_scope_name = &(scope_data.parent_scope);
            }
        }
    }

    fn update_var_declared(&mut self, var_name: &String, scope: &String){
        // let var = self.find_variable(var_name, scope);
        let scope_data = self.get_scope_data_mut(scope).expect("scope doesn't exist");
        scope_data.declared_variables.insert(var_name.clone().to_string());
    }

    fn get_type_size(&self, _type: &Type) -> u32 {
        if let Some(struct_data) = self.get_struct_data_from_type(_type){
            return struct_data.size
        }
        match _type{
            Type::Int => 1,
            Type::Ptr(_) => 1,
            Type::Void => 0,
            _ => panic!("invalid type")
        }
    }

    fn get_array_size(&self, item_type: &Type, dimentions: &Vec<u32>) -> u32{
        // this needs to be a member function because for example we could
        // have an array of structs, so we need access to the compiler's
        // data in order to know that size of each element in the array
        let mut size = 1;
        for x in dimentions.iter(){
            size *= x;
        }
        size * self.get_type_size(item_type)
    }

    fn get_decl_size(&self, decl: &Decl) -> u32{
        match decl{
            Decl::VarDecl(var_decl) => {
                self.get_type_size(&var_decl._type)
            },
            Decl::ArrayDecl(arr_decl) => {
                self.get_array_size(&arr_decl._type, &arr_decl.dimentions)
            }
        }
    }

    fn variable_data_from_decl(&self, decl: &Decl, local_or_arg: LocalOrArg, offset: &u32) -> VariableData{
        match decl{
            Decl::VarDecl(var_decl) => {
                let size = self.get_decl_size(decl);
                VariableData{
                    name: var_decl.name.clone(),
                    local_or_arg: local_or_arg,
                    var_type: VariableType::from(decl),
                    offset: *offset + size - 1,
                    size: size.clone(),
                }
            },
            Decl::ArrayDecl(arr_decl) => {
                let size = self.get_array_size(&arr_decl._type, &arr_decl.dimentions);
                VariableData{
                    name: arr_decl.name.clone(),
                    local_or_arg: local_or_arg,
                    var_type: VariableType::from(decl),
                    offset: *offset + size - 1,
                    size: size,
                }
            },
        }
    }
    fn register_scope(&mut self, scope_name: &String, statements: &Vec<Statement>, parent_scope_name: &String, parent_func_name: &String, current_var_offset: & mut u32){
        // collect variables
        let next_var_offset = current_var_offset;
        let mut variables = HashMap::new();
        for statement in statements.iter() {
            match statement{
                Statement::Decl(decl) => {
                    let var_data = self.variable_data_from_decl(&decl, LocalOrArg::Local, &next_var_offset.clone());
                    *next_var_offset += &var_data.size;
                    variables.insert(var_data.name.clone(), var_data);

                },
                Statement::Compound(comp) => {
                    let new_scope_name = &comp.code_loc;
                    self.register_scope(new_scope_name, &comp.items, scope_name, parent_func_name, next_var_offset);
                },
                Statement::If(if_stmt) => {
                    {
                        let iftrue_scope_name = &if_stmt.iftrue.code_loc;
                        self.register_scope(iftrue_scope_name, &if_stmt.iftrue.items, scope_name, parent_func_name, next_var_offset);
                    }
                    if let Some(ref iffalse) = if_stmt.iffalse{
                        let iffalse_scope_name = &iffalse.code_loc;
                        self.register_scope(iffalse_scope_name, &iffalse.items, scope_name, parent_func_name, next_var_offset);
                    }
                },
                Statement::WhileLoop(wl) => {
                    self.register_scope(&wl.code_loc, & wl.body.items, scope_name, parent_func_name, next_var_offset)
                },
                Statement::DoWhileLoop(dwl) => {
                    self.register_scope(&dwl.code_loc, & dwl.body.items, scope_name, parent_func_name, next_var_offset)
                },
                Statement::ForLoop(fl) => {
                    // we need to also collect variable declerations from initialization part of for loop
                    let mut for_init_vars = HashMap::new();
                    if let Some(init) = &fl.init{
                        for stmt in init.items.iter(){
                            match stmt{
                                Statement::Decl(decl) => {
                                    let var_data = self.variable_data_from_decl(&decl, LocalOrArg::Local, &next_var_offset.clone());
                                    *next_var_offset += var_data.size;
                                    for_init_vars.insert(var_data.name.clone(), var_data);
                                },
                                _ => {},
                            }
                        }
                    }
                    self.register_scope(&fl.code_loc, & fl.body.items, scope_name, parent_func_name, next_var_offset);
                    let for_body_scope = self.scope_to_data.get_mut(&fl.code_loc).unwrap();
                    for_body_scope.variables.extend(for_init_vars);

                }
                _ => {}
            }
            
        }

        let scope_data = ScopeData {
            name: scope_name.clone(),
            parent_scope: parent_scope_name.clone(),
            parent_func: parent_func_name.clone(),
            variables: variables,
            declared_variables: HashSet::new(),
            break_label: None,
            continue_label: None,
        };
        self.scope_to_data.insert(scope_name.clone(), scope_data);
    }

    fn register_func_decl(&mut self, func_decl: &FuncDecl){
        let mut args_types = Vec::new();
        for arg in func_decl.args.iter(){
            args_types.push(VariableType::from(arg));
        }
        let func_data = FuncData{
            decl_data: FuncDeclData{
                args_types: args_types,
                return_type: func_decl.ret_type.clone(),
            },
            body_data: None,
        };
        self.func_to_data.insert(func_decl.name.clone(), func_data);
    }

    fn register_func_body(&mut self, func_body: &Compound, func_decl: &FuncDecl, parent_scope: &String){
        let func_name = &func_decl.name;
        let mut vars_size : u32 = 0;
        self.register_scope(func_name, &func_body.items, parent_scope, func_name, &mut vars_size);

        let regs_used = vec![Register::R1, Register::R2];
        let funcret_type = func_decl.ret_type.clone();
        // insert local variables to scope's variables
        let mut cur_arg_offset : u32 = 0;
        let mut args_variables = HashMap::new();
        for arg in func_decl.args.iter(){
            let var_data = self.variable_data_from_decl(arg, LocalOrArg::Arg, &cur_arg_offset);
            cur_arg_offset += &var_data.size;
            args_variables.insert(var_data.name.clone(), var_data);
        }
        let func_scope = self.get_scope_data_mut(func_name).unwrap();
        // function args are automatically declared
        for (_, arg) in &args_variables{
            func_scope.declared_variables.insert(arg.name.clone());
        }
        func_scope.variables.extend(args_variables);
        

        let func_data = self.func_to_data.get_mut(&func_decl.name).expect("function not yet declared");
        func_data.body_data = Some(FuncBodyData{
            name: func_decl.name.clone(),
            regs_used: regs_used,
            local_vars_size: vars_size.clone(),
        });
    }

    fn register_struct(&mut self, struct_decl: &StructDecl){
        let mut items = LinkedHashMap::new();
        let mut cur_offset = 0;
        for (name, decl) in &struct_decl.items{
            let size = self.get_decl_size(decl);
            let var_data = VariableData {
                name: name.clone(),
                local_or_arg: LocalOrArg::Local,
                var_type: VariableType::from(decl),
                offset: cur_offset.clone(),
                size: size,
            };
            cur_offset += size;
            items.insert(name.clone(), var_data);
        }
        self.struct_to_data.insert(struct_decl.name.clone(), StructData{
            name: struct_decl.name.clone(),
            size: cur_offset.clone(),
            items,
        });
    }

    fn get_func_data(&self, func_name: &String) -> Option<&FuncData> {
        self.func_to_data.get(func_name)
    }

    fn _compile(&mut self, path_to_c_source: &str) -> Vec<String> {
        let program = preprocessor::preprocess(path_to_c_source);

        let mut tmpfile = tempfile::Builder::new().suffix(".c").tempfile().unwrap();
        write!(tmpfile, "{}", &program.as_str()).unwrap();

        let mut code: Vec<String> = Vec::new();
        let ast = AST::get_ast(tmpfile.path().to_str().unwrap());
        self.code_gen(AstNode::RootAstNode(&ast), &"_GLOBAL".to_string(), &mut code);

        code
    }

    pub fn compile(path_to_c_source: &str) -> String {
        let mut instance = Compiler::new();
        let instructions = instance._compile(path_to_c_source);
        instructions.join("\n")
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn find_variable(){
        let mut compiler = Compiler::new();
        compiler._compile("tests/compiler_test_data/variables/inputs/assign.c");
        let _a_var = compiler.find_variable(&"a".to_string(), &"main".to_string()).unwrap();
        let b_var = compiler.find_variable(&"b".to_string(), &"main".to_string());
        assert!(b_var.is_none());
    }
    #[test] #[ignore]
    fn find_nested_scope(){
        let mut compiler = Compiler::new();
        compiler._compile("tests/compiler_test_data/scopes/inputs/declare_block.c");
        println!("{:?}", compiler.scope_to_data);
        assert_eq!(compiler.scope_to_data.len(), 3);
        let block_scope = compiler.scope_to_data.get("tests/compiler_test_data/scopes/inputs/declare_block.c-2-1").unwrap();
        assert!(block_scope.variables.contains_key("i"));

    }

    #[test] #[ignore]

    fn find_break_continue_labels(){
        let mut compiler = Compiler::new();
        compiler._compile("tests/compiler_test_data/loops/inputs/while_multi_statement.c");
        println!("{:?}", compiler.scope_to_data);
        assert_eq!(compiler.scope_to_data.len(), 3);
        match compiler.find_break_continue_labels(&"tests/compiler_test_data/loops/inputs/while_multi_statement.c-5-5".to_string()){
            Some((break_label, continue_label)) => {
                assert_eq!(break_label, "WHILE_0_END");
                assert_eq!(continue_label, "WHILE_0_START");
            },
            _ => panic!()
        }
    }
    #[test]
    fn function_args(){
        let mut compiler = Compiler::new();
        compiler._compile("tests/compiler_test_data/functions/inputs/multi_arg.c");
        println!("{:?}", compiler.scope_to_data);
        let func_data = compiler.get_func_data(&"sub_3".to_string()).unwrap();
        let scope_data = compiler.get_scope_data(&"sub_3".to_string()).unwrap();
        match &func_data.decl_data.args_types[0]{
            VariableType::Regular{_type} => {
                assert!(matches!(_type, Type::Int));
            },
            _ => panic!(),
        }
        match &func_data.decl_data.args_types[1]{
            VariableType::Regular{_type} => {
                assert!(matches!(_type, Type::Int));
            },
            _ => panic!(),
        }
        match &func_data.decl_data.args_types[2]{
            VariableType::Regular{_type} => {
                assert!(matches!(_type, Type::Int));
            },
            _ => panic!(),
        }
        // assert_eq!(func_data.decl_data.args_types[1], "int");
        // assert_eq!(func_data.decl_data.args_types[2], "int");
        let x = scope_data.variables.get(&"x".to_string()).unwrap();
        assert_eq!(x.offset, 0);
        let y = scope_data.variables.get(&"y".to_string()).unwrap();
        assert_eq!(y.offset, 1);
        let z = scope_data.variables.get(&"z".to_string()).unwrap();
        assert_eq!(z.offset, 2);
    }

    #[test]
    fn struct_registration(){
        let mut compiler = Compiler::new();
        compiler._compile("tests/compiler_test_data/structs/inputs/1.c");
        let struct_data = compiler.struct_to_data.get("A").unwrap();
        assert_eq!(struct_data.name, "A");
        assert_eq!(struct_data.size, 3);
        let x = struct_data.items.get("x").unwrap();
        assert_eq!(x.name, "x");
        assert_eq!(x.offset, 0);
        assert_eq!(x.size, 1);
        if let VariableType::Regular{_type: t} = &x.var_type{
            assert!(matches!(t, Type::Int));
        } else{
            panic!();
        }
        assert_eq!(struct_data.items.get("y").unwrap().offset, 1);
        assert_eq!(struct_data.items.get("z").unwrap().offset, 2);
    }


}
