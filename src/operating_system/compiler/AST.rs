
extern crate serde_json;

use self::serde_json::Value as JsonNode;

use std::process::Command;
const PATH_TO_PY_EXEC : &str = "src/operating_system/compiler/parser/venv/bin/python";
const PATH_TO_PARSER : &str = "src/operating_system/compiler/parser/to_ast_json.py";

type AstError = ();

pub enum AstNode<'a>{
    RootAstNode(&'a RootAstNode),
    External(&'a External),
    FuncDef(&'a FuncDef),
    FuncDecl(&'a FuncDecl),
    Compound(&'a Compound),
    Statement(&'a Statement),
    Expression(&'a Expression),
    Constant(&'a Constant),
}

pub struct RootAstNode{
    pub externals: Vec<External>,
}

impl RootAstNode{
    fn from(node: &JsonNode) -> Result<RootAstNode, AstError>{
        let mut externals = Vec::new();
        for extNode in node["ext"].as_array().unwrap().iter(){
            externals.push(External::from(extNode)?);
        }
        Ok(RootAstNode{externals: externals})
    }
}

pub enum External{
    FuncDef(FuncDef),
}

impl External{
    fn from(node: &JsonNode) -> Result<External, AstError>{
        match node["_nodetype"].as_str().unwrap(){
            "FuncDef" => Ok(External::FuncDef(FuncDef::from(&node)?)),
            _ => Err(()),
        }
    }
}

pub struct FuncDef{
    pub body: Compound,
    pub decl: FuncDecl,
}
impl FuncDef{
    fn from(node: &JsonNode) -> Result<FuncDef, AstError>{
        Ok(FuncDef {
            body: Compound::from(&node["body"])?,
            decl: FuncDecl::from(&node["decl"])?,
        })
    }
}

pub struct FuncDecl{
    pub name: String,
    pub argsType: Vec<String>,
    pub retType: String,
}
impl FuncDecl{
    fn from(node: &JsonNode) -> Result<FuncDecl, AstError>{
        Ok(FuncDecl{
            name: node["name"].as_str().unwrap().to_string(),
            argsType: vec!["int".to_string()],
            retType: "int".to_string(),
        })
    }
}

pub struct Compound{
    pub items: Vec<Statement>,
}

impl Compound{
    fn from(node: &JsonNode) -> Result<Compound, AstError>{
        let mut statements = Vec::new();
        for statementNode in node["block_items"].as_array().unwrap().iter(){
            statements.push(Statement::from(&statementNode)?);
        }
        Ok(Compound{
            items: statements,
        })

    }
}

pub enum Statement{
    Return(Return),
    Decl(Decl),
}

impl Statement{
    fn from(node: &JsonNode) -> Result<Statement, AstError>{
        match node["_nodetype"].as_str().unwrap(){
            "Return" => Ok(Statement::Return(Return::from(&node)?)),
            _ => Err(()),
        }
    }
}

pub struct Return{
    pub expr: Expression,
}
pub struct Decl{
    pub name: String,
    pub _type: String,

}

impl Return{
    fn from(node: &JsonNode) -> Result<Return, AstError>{
        return Ok(Return{
            expr: Expression::from(&node["expr"])?,
        })
    }
}

pub enum Expression{
    Constant(Constant),
    BinaryOp(BinaryOp),
    UnaryOp(UnaryOp),
}

impl Expression{
    fn from(node: &JsonNode) -> Result<Expression, AstError>{
        match node["_nodetype"].as_str().unwrap(){
            "Constant" => Ok(Expression::Constant(Constant::from(&node)?)),
            "BinaryOp" => Ok(Expression::BinaryOp(BinaryOp::from(&node)?)),
            "UnaryOp" => Ok(Expression::UnaryOp(UnaryOp::from(&node)?)),
            _ => {
                println!("Invalid expression type:{}", node["_nodetype"].as_str().unwrap());
                Err(())
                },
        }
    }
}

pub struct Constant{
    pub _type: String,
    pub val: String,
}

impl Constant{
    fn from(node: &JsonNode) -> Result<Constant, AstError>{
        Ok(Constant{
            _type: node["type"].as_str().unwrap().to_string(),
            val: node["value"].as_str().unwrap().to_string(),
        })
    }
}

pub struct BinaryOp{
    pub opType: BinaryOpType,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

impl BinaryOp{
    fn from(node: &JsonNode) -> Result<BinaryOp, AstError>{
        let left = Box::new(Expression::from(&node["left"])?);
        let right = Box::new(Expression::from(&node["right"])?);
        let opType = BinaryOpType::from(&node["op"])?;
        Ok(BinaryOp{
            opType: opType,
            left: left,
            right: right,
        })
    }
}

#[derive(PartialEq, Debug)]
pub enum BinaryOpType{
   ADD,
   SUB,
   MUL,
   DIV,
   MOD,
   AND,
   OR,
   SHL,
   SHR,
   XOR,
}


impl BinaryOpType{
    fn from(node: &JsonNode) -> Result<BinaryOpType, AstError>{
        println!("BinaryOpType from:{}", node.as_str().unwrap());
        match node.as_str().unwrap(){
            "+" => Ok(BinaryOpType::ADD),
            "-" => Ok(BinaryOpType::SUB),
            "*" => Ok(BinaryOpType::MUL),
            "/" => Ok(BinaryOpType::DIV),
            "%" => Ok(BinaryOpType::MOD),
            "&" => Ok(BinaryOpType::AND),
            "|" => Ok(BinaryOpType::OR),
            "<<" => Ok(BinaryOpType::SHL),
            ">>" => Ok(BinaryOpType::SHR),
            "^" => Ok(BinaryOpType::XOR),
            _ => {
                println!("BinaryOpType from returning Err");
                Err(())
                },
        }
    }
    pub fn to_op(&self) -> Option<String>{
        match &self{
            BinaryOpType::ADD => Some("ADD".to_string()),
            BinaryOpType::SUB => Some("SUB".to_string()),
            BinaryOpType::MUL => Some("MUL".to_string()),
            BinaryOpType::DIV => Some("DIV".to_string()),
            BinaryOpType::MOD => Some("MOD".to_string()),
            BinaryOpType::AND => Some("AND".to_string()),
            BinaryOpType::OR => Some("OR".to_string()),
            BinaryOpType::SHL => Some("SHL".to_string()),
            BinaryOpType::SHR => Some("SHR".to_string()),
            BinaryOpType::XOR => Some("XOR".to_string()),
            _ => None,
        }
    }
}



pub struct UnaryOp{
    pub opType: UnaryOpType,
    pub expr: Box<Expression>,
}

impl UnaryOp{
    fn from(node: &JsonNode) -> Result<UnaryOp, AstError>{
        let expr = Box::new(Expression::from(&node["expr"])?);
        let opType = UnaryOpType::from(&node["op"])?;
        Ok(UnaryOp{
            opType: opType,
            expr: expr,
        })
    }
}

#[derive(PartialEq, Debug)]
pub enum UnaryOpType{
    NEG,
    NOT,
}


impl UnaryOpType{
    fn from(node: &JsonNode) -> Result<UnaryOpType, AstError>{
        println!("UnaryOpType from:{}", node.as_str().unwrap());
        match node.as_str().unwrap(){
            "!" => Ok(UnaryOpType::NOT),
            "-" => Ok(UnaryOpType::NEG),
            _ => {
                println!("UnaryOfType from returning Err");
                Err(())
                },
        }
    }
}


pub fn get_ast(path_to_c_source: &str) -> RootAstNode{
    let output = Command::new(PATH_TO_PY_EXEC)
                        .arg(PATH_TO_PARSER)
                        .arg(path_to_c_source)
                        .output()
                        .expect("Failed to execute c parser");

    let json_str = String::from_utf8(output.stdout).expect("Error decoding ast json bytes");

    let ast_json = serde_json::from_str(&json_str).expect("parser output is not JSON serializable");
    return RootAstNode::from(&ast_json).unwrap()

}
#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn main_const_return(){
        let ast_root = get_ast("tests/compiler_test_data/const_expressions/inputs/1.c");
        assert_eq!(ast_root.externals.len(), 1);
        match &ast_root.externals[0]{
            External::FuncDef(func_def) => {
                assert_eq!(func_def.decl.name, "main");
                assert_eq!(func_def.decl.retType, "int");
                match &func_def.body.items[0]{
                    Statement::Return(ret) => {
                        match &ret.expr{
                            Expression::Constant(c) =>{
                                assert_eq!(c._type, "int");
                                assert_eq!(c.val, "2");
                            },
                            _ => panic!(),
                        }

                    },
                    _ => panic!(),
                }
            },
            _ => panic!(),
        }
    }

    #[test]
    fn airth_ops(){
        let ast_root = get_ast("tests/compiler_test_data/arith_expressions/inputs/1plus1.c");
        assert_eq!(ast_root.externals.len(), 1);
        match &ast_root.externals[0]{
            External::FuncDef(func_def) => {
                assert_eq!(func_def.decl.name, "main");
                assert_eq!(func_def.decl.retType, "int");
                match &func_def.body.items[0]{
                    Statement::Return(ret) => {
                        match &ret.expr{
                            Expression::BinaryOp(bop) =>{
                                assert_eq!(bop.opType, BinaryOpType::ADD);
                                let left = &*bop.left;
                                if let Expression::Constant(left) = left{

                                }else{
                                    panic!();
                                }
                            },
                            _ => panic!(),
                        }

                    },
                    _ => panic!(),
                }
            },
            _ => panic!(),
        }

    }
}