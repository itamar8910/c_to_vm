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
        for ext_node in node["ext"].as_array().unwrap().iter() {
            externals.push(External::from(ext_node)?);
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
            }
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
    pub args: Vec<Decl>,
    pub ret_type: String,
}
impl FuncDecl {
    fn from(node: &JsonNode) -> Result<FuncDecl, AstError> {
        let mut args = Vec::new();
        match node["type"]["args"]{
            JsonNode::Object(_) => {
                for arg in node["type"]["args"]["params"].as_array().unwrap().iter(){
                    args.push(
                        Decl{
                            name: arg["name"].as_str().unwrap().to_string(),
                            _type: arg["type"]["type"]["names"].as_array().unwrap()[0].as_str().unwrap().to_string(),
                            init: None,
                        }
                    );
                }
            },
            _ => {},
        }
        Ok(FuncDecl {
            name: node["name"].as_str().unwrap().to_string(),
            args: args,
            ret_type: node["type"]["type"]["type"]["names"].as_array().unwrap()[0].as_str().unwrap().to_string(),
        })
    }
}

pub struct Compound {
    pub items: Vec<Statement>,
    pub code_loc: String, // needed for scope id
}

impl Compound {
    fn from(node: &JsonNode) -> Result<Compound, AstError> {
        let mut statements = Vec::new();
        let node_type = node["_nodetype"].as_str().unwrap();
        if node_type != "EmptyStatement"{
            if node_type == "DeclList"{
                // we treat DeclList as a compound, because a declaration is also a statement
                for decl_node in node["decls"].as_array().unwrap().iter() {
                    statements.push(Statement::from(&decl_node)?);
                }
            }
            else{
                match node["block_items"] {
                    JsonNode::Null => {
                        statements.push(Statement::from(&node)?);
                    }
                    _ => {
                        for statement_node in node["block_items"].as_array().unwrap().iter() {
                            statements.push(Statement::from(&statement_node)?);
                        }
                    }
                }
            }
        }
        Ok(Compound {
             items: statements,
             code_loc: node["coord"].as_str().unwrap().to_string().replace(":", "-"),
        })

    }
}

pub enum Statement {
    Return(Return),
    Decl(Decl),
    Assignment(Assignment),
    Expression(Expression),
    If(If),
    Compound(Compound),
    WhileLoop(WhileLoop),
    DoWhileLoop(DoWhileLoop),
    ForLoop(ForLoop),
    Break,
    Continue,
}

impl Statement {
    fn from(node: &JsonNode) -> Result<Statement, AstError> {
        match node["_nodetype"].as_str().unwrap() {
            "Return" => Ok(Statement::Return(Return::from(&node)?)),
            "Decl" => Ok(Statement::Decl(Decl::from(&node)?)),
            "Assignment" => Ok(Statement::Assignment(Assignment::from(&node)?)),
            "If" => Ok(Statement::If(If::from(&node)?)),
            "Compound" | "EmptyStatement"=> Ok(Statement::Compound(Compound::from(&node)?)),
            "While" => Ok(Statement::WhileLoop(WhileLoop::from(&node)?)),
            "DoWhile" => Ok(Statement::DoWhileLoop(DoWhileLoop::from(&node)?)),
            "For" => Ok(Statement::ForLoop(ForLoop::from(&node)?)),
            "Break" => Ok(Statement::Break),
            "Continue" => Ok(Statement::Continue),
            _ => {
                Ok(Statement::Expression(Expression::from(&node)?))
            }
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
    TernaryOp(TernaryOp),
    FuncCall(FuncCall),
}

impl Expression {
    fn from(node: &JsonNode) -> Result<Expression, AstError> {
        match node["_nodetype"].as_str().unwrap() {
            "Constant" => Ok(Expression::Constant(Constant::from(&node)?)),
            "BinaryOp" => Ok(Expression::BinaryOp(BinaryOp::from(&node)?)),
            "UnaryOp" => Ok(Expression::UnaryOp(UnaryOp::from(&node)?)),
            "ID" => Ok(Expression::ID(ID::from(&node)?)),
            "Assignment" => Ok(Expression::Assignment(Assignment::from(&node)?)),
            "TernaryOp" => Ok(Expression::TernaryOp(TernaryOp::from(&node)?)),
            "FuncCall" => Ok(Expression::FuncCall(FuncCall::from(&node)?)),
            _ => {
                panic!(format!(
                    "Invalid expression type:{}",
                    node["_nodetype"].as_str().unwrap())
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
    pub op_type: BinaryopType,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

impl BinaryOp {
    fn from(node: &JsonNode) -> Result<BinaryOp, AstError> {
        let left = Box::new(Expression::from(&node["left"])?);
        let right = Box::new(Expression::from(&node["right"])?);
        let op_type = BinaryopType::from(&node["op"])?;
        Ok(BinaryOp {
            op_type: op_type,
            left: left,
            right: right,
        })
    }
}

#[derive(PartialEq, Debug)]
pub enum BinaryopType {
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
    LogicalAnd,
    LogicalOr,
    LT,
    LTEQ,
    GT,
    GTEQ,
}

impl BinaryopType {
    fn _from(s: &str) -> Result<BinaryopType, AstError> {
        println!("BinaryopType from:{}", s);
        match s {
            "+" => Ok(BinaryopType::ADD),
            "-" => Ok(BinaryopType::SUB),
            "*" => Ok(BinaryopType::MUL),
            "/" => Ok(BinaryopType::DIV),
            "%" => Ok(BinaryopType::MOD),
            "&" => Ok(BinaryopType::AND),
            "|" => Ok(BinaryopType::OR),
            "<<" => Ok(BinaryopType::SHL),
            ">>" => Ok(BinaryopType::SHR),
            "^" => Ok(BinaryopType::XOR),
            "==" => Ok(BinaryopType::EQ),
            "!=" => Ok(BinaryopType::NEQ),
            "&&" => Ok(BinaryopType::LogicalAnd),
            "||" => Ok(BinaryopType::LogicalOr),
            "<" => Ok(BinaryopType::LT),
            "<=" => Ok(BinaryopType::LTEQ),
            ">" => Ok(BinaryopType::GT),
            ">=" => Ok(BinaryopType::GTEQ),
            _ => {
                println!("BinaryopType from returning Err");
                Err(())
            }
        }
    }
    fn from(node: &JsonNode) -> Result<BinaryopType, AstError> {
        BinaryopType::_from(&node.as_str().unwrap())
    }
    pub fn to_op(&self) -> Option<String> {
        match &self {
            BinaryopType::ADD => Some("ADD".to_string()),
            BinaryopType::SUB => Some("SUB".to_string()),
            BinaryopType::MUL => Some("MUL".to_string()),
            BinaryopType::DIV => Some("DIV".to_string()),
            BinaryopType::MOD => Some("MOD".to_string()),
            BinaryopType::AND => Some("AND".to_string()),
            BinaryopType::OR => Some("OR".to_string()),
            BinaryopType::SHL => Some("SHL".to_string()),
            BinaryopType::SHR => Some("SHR".to_string()),
            BinaryopType::XOR => Some("XOR".to_string()),
            _ => None,
        }
    }
}

pub struct UnaryOp {
    pub op_type: UnaryopType,
    pub expr: Box<Expression>,
}

impl UnaryOp {
    fn from(node: &JsonNode) -> Result<UnaryOp, AstError> {
        let expr = Box::new(Expression::from(&node["expr"])?);
        let op_type = UnaryopType::from(&node["op"])?;
        Ok(UnaryOp {
            op_type: op_type,
            expr: expr,
        })
    }
}

#[derive(PartialEq, Debug)]
pub enum UnaryopType {
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

impl UnaryopType {
    fn from(node: &JsonNode) -> Result<UnaryopType, AstError> {
        println!("UnaryopType from:{}", node.as_str().unwrap());
        match node.as_str().unwrap() {
            "!" => Ok(UnaryopType::NOT),
            "-" => Ok(UnaryopType::NEG),
            "p++" => Ok(UnaryopType::XPP),
            "++" => Ok(UnaryopType::PPX),
            "p--" => Ok(UnaryopType::XMM),
            "--" => Ok(UnaryopType::MMX),
            _ => {
                panic!("Unkown Unary type:{}", node.as_str().unwrap());
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
    pub op: Option<BinaryopType>, // e.g for += assignment, this will be PLUS
}

impl AssignmentOp {
    fn from(node: &JsonNode) -> Result<AssignmentOp, AstError> {
        let op_str = node["op"].as_str().unwrap().to_string();
        match op_str.len() {
            1 => Ok(AssignmentOp { op: None }),
            2 | 3 => {
                // let op_char = op_str.chars().next().unwrap();
                let eq_index = op_str
                    .as_str()
                    .find('=')
                    .expect("no = sign in assignment with op");
                let op_part: String = op_str.chars().take(eq_index).collect();
                Ok(AssignmentOp {
                    op: Some(BinaryopType::_from(&op_part.as_str())?),
                })
            }
            _ => panic!("invalid assignment op string:{}", op_str),
        }
    }
}

pub struct If {
    pub cond: Expression,
    pub iftrue: Box<Compound>,
    pub iffalse: Option<Box<Compound>>,
    pub code_loc: String, // needed for scope id
}

impl If {
    fn from(node: &JsonNode) -> Result<If, AstError> {
        let iffalse = maybe_get_boxed_compound(node, "iffalse");
        Ok(If {
            cond: Expression::from(&node["cond"])?,
            iftrue: Box::new(Compound::from(&node["iftrue"])?),
            iffalse: iffalse,
            code_loc: node["coord"].as_str().unwrap().to_string().replace(":","-"),
        })
    }
}

pub struct TernaryOp {
    pub cond: Box<Expression>,
    pub iftrue: Box<Expression>,
    pub iffalse: Box<Expression>,
}

impl TernaryOp {
    fn from(node: &JsonNode) -> Result<TernaryOp, AstError> {
        Ok(TernaryOp {
            cond: Box::new(Expression::from(&node["cond"])?),
            iftrue: Box::new(Expression::from(&node["iftrue"])?),
            iffalse: Box::new(Expression::from(&node["iffalse"])?),
        })
    }
}

pub struct WhileLoop {
    pub cond: Expression,
    pub body: Box<Compound>,
    pub code_loc: String, // needed for scope id
}

impl WhileLoop {
    fn from(node: &JsonNode) -> Result<WhileLoop, AstError> {
        Ok(WhileLoop{
            cond: Expression::from(&node["cond"])?,
            body: Box::new(Compound::from(&node["stmt"])?),
            code_loc: node["coord"].as_str().unwrap().to_string().replace(":","-"),
        })
    }
}

pub struct DoWhileLoop {
    pub cond: Expression,
    pub body: Box<Compound>,
    pub code_loc: String, // needed for scope id
}

impl DoWhileLoop {
    fn from(node: &JsonNode) -> Result<DoWhileLoop, AstError> {
        Ok(DoWhileLoop{
            cond: Expression::from(&node["cond"])?,
            body: Box::new(Compound::from(&node["stmt"])?),
            code_loc: node["coord"].as_str().unwrap().to_string().replace(":","-"),
        })
    }
}

pub struct ForLoop{
    pub cond: Option<Expression>,
    pub init: Option<Box<Compound>>,
    pub body: Box<Compound>,
    pub next: Option<Box<Compound>>,
    pub code_loc: String, // needed for scope id
}

fn maybe_get_boxed_compound(node: &JsonNode, key: &str) -> Option<Box<Compound>>{
    match &node[key] {
        JsonNode::Object(_) => Some(Box::new(Compound::from(&node[key]).unwrap())),
        JsonNode::Null => None,
        _ => panic!("invalid type for optional compound"),
    }

}

impl ForLoop {
    fn from(node: &JsonNode) -> Result<ForLoop, AstError> {
        println!("creating for loop");
        Ok(ForLoop{
            cond: 
                match &node["cond"]{
                    JsonNode::Object(_) => Some(Expression::from(&node["cond"]).unwrap()),
                    JsonNode::Null => None,
                    _ => panic!("unexpected JSON type for cond")
                },
            init: maybe_get_boxed_compound(node, "init"),
            body: Box::new(Compound::from(&node["stmt"])?),
            next: maybe_get_boxed_compound(node, "next"),
            code_loc: node["coord"].as_str().unwrap().to_string().replace(":","-"),
        })
    }
}

pub struct FuncCall{
    pub name: String,
    pub args: Vec<Box<Expression>>,
}

impl FuncCall {
    fn from(node: &JsonNode) -> Result<FuncCall, AstError> {
        let mut args = Vec::new();
        for expr in node["args"]["exprs"].as_array().unwrap().iter(){
            args.push(Box::new(Expression::from(expr)?));
        }
        Ok(FuncCall{
            name: node["name"]["name"].as_str().unwrap().to_string(),
            args: args,
        })
    }
}

pub fn get_ast(path_to_c_source: &str) -> RootAstNode {
    assert!(path_to_c_source.ends_with(".c"));
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
                assert_eq!(func_def.decl.ret_type, "int");
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
                assert_eq!(func_def.decl.ret_type, "int");
                match &func_def.body.items[0] {
                    Statement::Return(ret) => match &ret.expr {
                        Expression::BinaryOp(bop) => {
                            assert_eq!(bop.op_type, BinaryopType::ADD);
                            let left = &*bop.left;
                            if let Expression::Constant(_left) = left {

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
                assert_eq!(func_def.decl.ret_type, "int");
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

    #[test]
    fn if_without_else() {
        let ast_root = get_ast("tests/compiler_test_data/if_statement/inputs/if_taken.c");
        assert_eq!(ast_root.externals.len(), 1);
        match &ast_root.externals[0] {
            External::FuncDef(func_def) => {
                assert_eq!(func_def.decl.name, "main");
                assert_eq!(func_def.decl.ret_type, "int");
                match &func_def.body.items[2] {
                    Statement::If(if_stmt) => {
                        if let Expression::ID(id) = &if_stmt.cond {
                            assert_eq!(id.name, "a");
                        } else {
                            panic!();
                        }
                        let iftrue = &**&if_stmt.iftrue; // unbox statement
                        let iftrue = &iftrue.items[0];
                        if let Statement::Assignment(_ass) = iftrue {
                        } else {
                            panic!();
                        }
                        match &if_stmt.iffalse {
                            None => {}
                            _ => panic!(),
                        }
                    }
                    _ => panic!(),
                }
            }
            _ => panic!(),
        }
    }
    #[test]
    fn compound_if() {
        let ast_root = get_ast("tests/compiler_test_data/if_statement/inputs/if_compound.c");
        assert_eq!(ast_root.externals.len(), 1);
        match &ast_root.externals[0] {
            External::FuncDef(func_def) => {
                assert_eq!(func_def.decl.name, "main");
                assert_eq!(func_def.decl.ret_type, "int");
                match &func_def.body.items[1] {
                    Statement::If(if_stmt) => {
                        if let Expression::BinaryOp(bop) = &if_stmt.cond {
                            match bop.op_type {
                                BinaryopType::GT => {}
                                _ => panic!(),
                            }
                        } else {
                            panic!();
                        }
                        let iftrue = &*if_stmt.iftrue; // unbox statement, then get reference to unboxed value
                        if let Statement::Assignment(_ass) = &iftrue.items[0] {
                        } else {
                            panic!();
                        }
                        if let Statement::Assignment(_ass) = &iftrue.items[1] {
                        } else {
                            panic!();
                        }
                        match &if_stmt.iffalse {
                            None => {}
                            _ => panic!(),
                        }
                    }
                    _ => panic!(),
                }
            }
            _ => panic!(),
        }
    }
    #[test]
    fn ternary() {
        let ast_root = get_ast("tests/compiler_test_data/ternary_expression/inputs/ternary.c");
        assert_eq!(ast_root.externals.len(), 1);
        match &ast_root.externals[0] {
            External::FuncDef(func_def) => {
                assert_eq!(func_def.decl.name, "main");
                assert_eq!(func_def.decl.ret_type, "int");
                match &func_def.body.items[1] {
                    Statement::Return(ret) => {
                        if let Expression::TernaryOp(top) = &ret.expr {
                            if let Expression::BinaryOp(ref bop) = **&top.cond {
                                match bop.op_type {
                                    BinaryopType::GT => {}
                                    _ => panic!(),
                                }
                            }
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
    #[test]
    fn while_loop(){
        let ast_root = get_ast("tests/compiler_test_data/loops/inputs/while_single_statement.c");
        match &ast_root.externals[0] {
            External::FuncDef(func_def) => {
                match &func_def.body.items[1] {
                    Statement::WhileLoop(while_stmt) => {
                        match &while_stmt.cond{
                            Expression::BinaryOp(bop) => {
                                match &bop.op_type{
                                    BinaryopType::LT => {},
                                    _ => panic!(),
                                }
                            },
                            _ => panic!()
                        };
                        match &while_stmt.body.items[0]{
                            Statement::Assignment(ass) => {},
                            _ => panic!(),
                        }
                    }
                    _ => panic!()
                }
            },
            _ => panic!()
        }

    }
    #[test]
    fn for_loop(){
        let ast_root = get_ast("tests/compiler_test_data/loops/inputs/for.c");
        match &ast_root.externals[0] {
            External::FuncDef(func_def) => {
                match &func_def.body.items[1] {
                    Statement::ForLoop(fl) => {
                        match &fl.cond{
                            Some(Expression::BinaryOp(bop)) => {
                                match &bop.op_type{
                                    BinaryopType::LT => {},
                                    _ => panic!(),
                                }
                            },
                            _ => panic!()
                        };
                        match &fl.body.items[0]{
                            Statement::Assignment(ass) => {},
                            _ => panic!(),
                        };
                        let init = &fl.init.as_ref().unwrap();
                        match &init.items[0]{
                            Statement::Assignment(ass) => {},
                            _ => panic!(),
                        };
                        let next = &fl.next.as_ref().unwrap();
                        match &next.items[0]{
                            Statement::Assignment(ass) => {
                                let right = & *ass.rvalue;
                                match right{
                                    Expression::BinaryOp(bop) => {
                                        match bop.op_type{
                                            BinaryopType::ADD => {},
                                            _ => panic!()
                                        }
                                    },
                                    _ => panic!()
                                };
                            },
                            _ => panic!(),
                        };
                    }
                    _ => panic!()
                }
            },
            _ => panic!()
        }

    }

    #[test]
    fn empty_for_loop(){
        let ast_root = get_ast("tests/compiler_test_data/loops/inputs/for_empty.c");
        match &ast_root.externals[0] {
            External::FuncDef(func_def) => {
                match &func_def.body.items[1] {
                    Statement::ForLoop(fl) => {
                        match &fl.init{
                            None => {},
                            _ => panic!(),
                        };
                        match &fl.cond{
                            None => {},
                            _ => panic!(),
                        };
                        match &fl.next{
                            None => {},
                            _ => panic!(),
                        };
                        assert_eq!(fl.body.items.len(), 2);
                    },
                    _ => panic!(),
                }
            }
        }
    }

    #[test]
    fn for_loop_init_decl(){
        let ast_root = get_ast("tests/compiler_test_data/loops/inputs/break.c");
        match &ast_root.externals[0] {
            External::FuncDef(func_def) => {
                match &func_def.body.items[1] {
                    Statement::ForLoop(fl) => {
                        let init = &fl.init.as_ref().unwrap();
                        match &init.items[0]{
                            Statement::Decl(d) => {
                                assert_eq!(d.name, "i");
                            },
                            _ => panic!()
                        }
                        assert_eq!(fl.body.items.len(), 2);
                    },
                    _ => panic!(),
                }
            }
        }
    }
    #[test]
    fn func_decl_args(){
        let ast_root = get_ast("tests/compiler_test_data/functions/inputs/multi_arg.c");
        match &ast_root.externals[0] {
            External::FuncDef(func_def) => {
                let func_decl = &func_def.decl;
                let args = &func_decl.args;
                assert_eq!(args[0].name, "x");
                assert_eq!(args[0]._type, "int");
                assert_eq!(args[1].name, "y");
                assert_eq!(args[1]._type, "int");
                assert_eq!(args[2].name, "z");
                assert_eq!(args[2]._type, "int");
            },
            _ => panic!(),
        }
    }
    #[test]
    fn func_call_args(){
        let ast_root = get_ast("tests/compiler_test_data/functions/inputs/multi_arg.c");
        match &ast_root.externals[1] {
            External::FuncDef(func_def) => {
                match &func_def.body.items[0]{
                    Statement::Return(ret) => {
                        match &ret.expr{
                            Expression::FuncCall(func_call) => {
                                assert_eq!(func_call.name, "sub_3");
                                assert_eq!(func_call.args.len(), 3);
                                let arg0 = &*func_call.args[0];
                                match arg0{
                                    Expression::Constant(c) => {
                                        assert_eq!(c.val, "10");
                                    },
                                    _ => panic!()
                                }
                            },
                            _ => panic!()
                        }
                    },
                    _ => panic!()
                }
            },
            _ => panic!(),
        }
    }
}
