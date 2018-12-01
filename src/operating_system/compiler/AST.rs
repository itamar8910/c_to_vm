extern crate serde_json;

use self::serde_json::Value as JsonNode;

use std::process::Command;
const PATH_TO_PY_EXEC: &str = "src/operating_system/compiler/parser/venv/bin/python";
const PATH_TO_PARSER: &str = "src/operating_system/compiler/parser/to_ast_json.py";

type AstError = ();

pub enum AstNode<'a> {
    RootAstNode(&'a RootAstNode),
    External(&'a External),
    FuncDef(&'a FuncDef),
    FuncDecl(&'a FuncDecl),
    Compound(&'a Compound),
    Statement(&'a Statement),
    Expression(&'a Expression),
    Constant(&'a Constant),
}

pub struct RootAstNode {
    pub externals: Vec<External>,
}

impl RootAstNode {
    fn from(node: &JsonNode) -> Result<RootAstNode, AstError> {
        let mut externals = Vec::new();
        for extNode in node["ext"].as_array().unwrap().iter() {
            externals.push(External::from(extNode)?);
        }
        Ok(RootAstNode {
            externals: externals,
        })
    }
}

pub enum External {
    FuncDef(FuncDef),
}

impl External {
    fn from(node: &JsonNode) -> Result<External, AstError> {
        match node["_nodetype"].as_str().unwrap() {
            "FuncDef" => Ok(External::FuncDef(FuncDef::from(&node)?)),
            _ => {
                panic!("Invalid external");
                Err(())
                },
        }
    }
}

pub struct FuncDef {
    pub body: Compound,
    pub decl: FuncDecl,
}
impl FuncDef {
    fn from(node: &JsonNode) -> Result<FuncDef, AstError> {
        Ok(FuncDef {
            body: Compound::from(&node["body"])?,
            decl: FuncDecl::from(&node["decl"])?,
        })
    }
}

pub struct FuncDecl {
    pub name: String,
    pub argsType: Vec<String>,
    pub retType: String,
}
impl FuncDecl {
    fn from(node: &JsonNode) -> Result<FuncDecl, AstError> {
        Ok(FuncDecl {
            name: node["name"].as_str().unwrap().to_string(),
            argsType: vec!["int".to_string()],
            retType: "int".to_string(),
        })
    }
}

pub struct Compound {
    pub items: Vec<Statement>,
}

impl Compound {
    fn from(node: &JsonNode) -> Result<Compound, AstError> {
        let mut statements = Vec::new();
        for statementNode in node["block_items"].as_array().unwrap().iter() {
            statements.push(Statement::from(&statementNode)?);
        }
        Ok(Compound { items: statements })
    }
}

pub enum Statement {
    Return(Return),
    Decl(Decl),
    Assignment(Assignment),
    Expression(Expression),
}

impl Statement {
    fn from(node: &JsonNode) -> Result<Statement, AstError> {
        match node["_nodetype"].as_str().unwrap() {
            "Return" => Ok(Statement::Return(Return::from(&node)?)),
            "Decl" => Ok(Statement::Decl(Decl::from(&node)?)),
            "Assignment" => Ok(Statement::Assignment(Assignment::from(&node)?)),
            _ => {
                Ok(Statement::Expression(Expression::from(&node)?))
                // panic!("Invalid statement type: {}", node["_nodetype"].as_str().unwrap());
                // Err(())
                },
        }
    }
}

pub struct Return {
    pub expr: Expression,
}

impl Return {
    fn from(node: &JsonNode) -> Result<Return, AstError> {
        return Ok(Return {
            expr: Expression::from(&node["expr"])?,
        });
    }
}

pub struct Decl {
    pub name: String,
    pub _type: String,
    pub init: Option<Expression>,
}

impl Decl {
    fn from(node: &JsonNode) -> Result<Decl, AstError> {
        let name = node["name"].as_str().unwrap().to_string();
        let _type = "int".to_string(); // TODO: generalize
        let init = match node["init"] {
            JsonNode::Object(_) => Some(Expression::from(&node["init"])?),
            JsonNode::Null => None,
            _ => panic!("Invalid decl init type"),
        };
        Ok(Decl {
            name: name,
            _type: _type,
            init: init,
        })
    }
}

pub enum Expression {
    Constant(Constant),
    BinaryOp(BinaryOp),
    UnaryOp(UnaryOp),
    ID(ID),
    Assignment(Assignment),
}

impl Expression {
    fn from(node: &JsonNode) -> Result<Expression, AstError> {
        match node["_nodetype"].as_str().unwrap() {
            "Constant" => Ok(Expression::Constant(Constant::from(&node)?)),
            "BinaryOp" => Ok(Expression::BinaryOp(BinaryOp::from(&node)?)),
            "UnaryOp" => Ok(Expression::UnaryOp(UnaryOp::from(&node)?)),
            "ID" => Ok(Expression::ID(ID::from(&node)?)),
            "Assignment" => Ok(Expression::Assignment(Assignment::from(&node)?)),
            _ => {
                println!(
                    "Invalid expression type:{}",
                    node["_nodetype"].as_str().unwrap()
                );
                Err(())
            }
        }
    }
}

pub struct Constant {
    pub _type: String,
    pub val: String,
}

impl Constant {
    fn from(node: &JsonNode) -> Result<Constant, AstError> {
        Ok(Constant {
            _type: node["type"].as_str().unwrap().to_string(),
            val: node["value"].as_str().unwrap().to_string(),
        })
    }
}

pub struct BinaryOp {
    pub opType: BinaryOpType,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

impl BinaryOp {
    fn from(node: &JsonNode) -> Result<BinaryOp, AstError> {
        let left = Box::new(Expression::from(&node["left"])?);
        let right = Box::new(Expression::from(&node["right"])?);
        let opType = BinaryOpType::from(&node["op"])?;
        Ok(BinaryOp {
            opType: opType,
            left: left,
            right: right,
        })
    }
}

#[derive(PartialEq, Debug)]
pub enum BinaryOpType {
    // arithmetical
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
    // boolean
    EQ,
    NEQ,
    LOGICAL_AND,
    LOGICAL_OR,
    LT,
    LTEQ,
    GT,
    GTEQ,
}

impl BinaryOpType {
    fn _from(s: &str) -> Result<BinaryOpType, AstError> {
        println!("BinaryOpType from:{}", s);
        match s {
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
            "==" => Ok(BinaryOpType::EQ),
            "!=" => Ok(BinaryOpType::NEQ),
            "&&" => Ok(BinaryOpType::LOGICAL_AND),
            "||" => Ok(BinaryOpType::LOGICAL_OR),
            "<" => Ok(BinaryOpType::LT),
            "<=" => Ok(BinaryOpType::LTEQ),
            ">" => Ok(BinaryOpType::GT),
            ">=" => Ok(BinaryOpType::GTEQ),
            _ => {
                println!("BinaryOpType from returning Err");
                Err(())
            }
        }
    }
    fn from(node: &JsonNode) -> Result<BinaryOpType, AstError> {
        BinaryOpType::_from(&node.as_str().unwrap())
    }
    pub fn to_op(&self) -> Option<String> {
        match &self {
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

pub struct UnaryOp {
    pub opType: UnaryOpType,
    pub expr: Box<Expression>,
}

impl UnaryOp {
    fn from(node: &JsonNode) -> Result<UnaryOp, AstError> {
        let expr = Box::new(Expression::from(&node["expr"])?);
        let opType = UnaryOpType::from(&node["op"])?;
        Ok(UnaryOp {
            opType: opType,
            expr: expr,
        })
    }
}

#[derive(PartialEq, Debug)]
pub enum UnaryOpType {
    NEG,
    NOT,
    XPP, // x++
    PPX, // ++x
    XMM, // x--
    MMX, // --x

}

pub struct ID {
    pub name: String,
}

impl ID {
    fn from(node: &JsonNode) -> Result<ID, AstError> {
        Ok(ID {
            name: node["name"].as_str().unwrap().to_string(),
        })
    }
}

impl UnaryOpType {
    fn from(node: &JsonNode) -> Result<UnaryOpType, AstError> {
        println!("UnaryOpType from:{}", node.as_str().unwrap());
        match node.as_str().unwrap() {
            "!" => Ok(UnaryOpType::NOT),
            "-" => Ok(UnaryOpType::NEG),
            "p++" => Ok(UnaryOpType::XPP),
            "++" => Ok(UnaryOpType::PPX),
            "p--" => Ok(UnaryOpType::XMM),
            "--" => Ok(UnaryOpType::MMX),
            _ => {
                panic!("Unkown Unary type:{}", node.as_str().unwrap());
                Err(())
            }
        }
    }
}

pub struct Assignment {
    pub op: AssignmentOp,
    pub lvalue: Box<Expression>,
    pub rvalue: Box<Expression>,
}

impl Assignment {
    fn from(node: &JsonNode) -> Result<Assignment, AstError> {
        let op = AssignmentOp::from(&node)?;
        let lvalue = Box::new(Expression::from(&node["lvalue"])?);
        let rvalue = Box::new(Expression::from(&node["rvalue"])?);
        Ok(Assignment {
            op: op,
            lvalue: lvalue,
            rvalue: rvalue,
        })
    }
}

pub struct AssignmentOp {
    pub op: Option<BinaryOpType>, // e.g for += assignment, this will be PLUS
}

impl AssignmentOp {
    fn from(node: &JsonNode) -> Result<AssignmentOp, AstError> {
        let op_str = node["op"].as_str().unwrap().to_string();
        match op_str.len() {
            1 => Ok(AssignmentOp { op: None }),
            2 | 3 => {
                // let op_char = op_str.chars().next().unwrap();
                let eq_index = op_str.as_str().find('=').expect("no = sign in assignment with op");
                let op_part : String = op_str.chars().take(eq_index).collect();
                Ok(AssignmentOp {
                    op: Some(BinaryOpType::_from(&op_part.as_str())?),
                })
            }
            _ => panic!("invalid assignment op string:{}", op_str),
        }
    }
}

pub fn get_ast(path_to_c_source: &str) -> RootAstNode {
    let output = Command::new(PATH_TO_PY_EXEC)
        .arg(PATH_TO_PARSER)
        .arg(path_to_c_source)
        .output()
        .expect("Failed to execute c parser");

    let json_str = String::from_utf8(output.stdout).expect("Error decoding ast json bytes");

    let ast_json = serde_json::from_str(&json_str).expect("parser output is not JSON serializable");
    return RootAstNode::from(&ast_json).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main_const_return() {
        let ast_root = get_ast("tests/compiler_test_data/const_expressions/inputs/1.c");
        assert_eq!(ast_root.externals.len(), 1);
        match &ast_root.externals[0] {
            External::FuncDef(func_def) => {
                assert_eq!(func_def.decl.name, "main");
                assert_eq!(func_def.decl.retType, "int");
                match &func_def.body.items[0] {
                    Statement::Return(ret) => match &ret.expr {
                        Expression::Constant(c) => {
                            assert_eq!(c._type, "int");
                            assert_eq!(c.val, "2");
                        }
                        _ => panic!(),
                    },
                    _ => panic!(),
                }
            }
            _ => panic!(),
        }
    }

    #[test]
    fn airth_ops() {
        let ast_root = get_ast("tests/compiler_test_data/arith_expressions/inputs/1plus1.c");
        assert_eq!(ast_root.externals.len(), 1);
        match &ast_root.externals[0] {
            External::FuncDef(func_def) => {
                assert_eq!(func_def.decl.name, "main");
                assert_eq!(func_def.decl.retType, "int");
                match &func_def.body.items[0] {
                    Statement::Return(ret) => match &ret.expr {
                        Expression::BinaryOp(bop) => {
                            assert_eq!(bop.opType, BinaryOpType::ADD);
                            let left = &*bop.left;
                            if let Expression::Constant(left) = left {

                            } else {
                                panic!();
                            }
                        }
                        _ => panic!(),
                    },
                    _ => panic!(),
                }
            }
            _ => panic!(),
        }
    }
    #[test]
    fn var_init() {
        let ast_root = get_ast("tests/compiler_test_data/variables/inputs/initialize.c");
        assert_eq!(ast_root.externals.len(), 1);
        match &ast_root.externals[0] {
            External::FuncDef(func_def) => {
                assert_eq!(func_def.decl.name, "main");
                assert_eq!(func_def.decl.retType, "int");
                match &func_def.body.items[0] {
                    Statement::Decl(decl) => {
                        assert_eq!(decl.name, "a");
                        assert_eq!(decl._type, "int");
                        if let Some(Expression::Constant(c)) = &decl.init {
                            assert_eq!(c.val, "2");
                        } else {
                            panic!();
                        }
                    }
                    _ => panic!(),
                }
            }
            _ => panic!(),
        }
    }
}
