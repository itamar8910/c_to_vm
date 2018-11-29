
extern crate serde_json;

use self::serde_json::Value as JsonNode;

use std::process::Command;
const PATH_TO_PY_EXEC : &str = "src/operating_system/compiler/parser/venv/bin/python";
const PATH_TO_PARSER : &str = "src/operating_system/compiler/parser/to_ast_json.py";

type AstError = ();


pub struct RootAstNode{
    externals: Vec<External>,
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

enum External{
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

struct FuncDef{
    body: Compound,
    decl: FuncDecl,
}
impl FuncDef{
    fn from(node: &JsonNode) -> Result<FuncDef, AstError>{
        Ok(FuncDef {
            body: Compound::from(&node["body"])?,
            decl: FuncDecl::from(&node["decl"])?,
        })
    }
}

struct FuncDecl{
    name: String,
    argsType: Vec<String>,
    retType: String,
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

struct Compound{
    items: Vec<Statement>,
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

enum Statement{
    Return(Return),
}

impl Statement{
    fn from(node: &JsonNode) -> Result<Statement, AstError>{
        match node["_nodetype"].as_str().unwrap(){
            "Return" => Ok(Statement::Return(Return::from(&node)?)),
            _ => Err(()),
        }
    }
}

struct Return{
    expr: Expression,
}

impl Return{
    fn from(node: &JsonNode) -> Result<Return, AstError>{
        return Ok(Return{
            expr: Expression::from(&node["expr"])?,
        })
    }
}

enum Expression{
    Constant(Constant),
}

impl Expression{
    fn from(node: &JsonNode) -> Result<Expression, AstError>{
        match node["_nodetype"].as_str().unwrap(){
            "Constant" => Ok(Expression::Constant(Constant::from(&node)?)),
            _ => Err(()),
        }

    }
}

struct Constant{
    _type: String,
    val: String,
}

impl Constant{
    fn from(node: &JsonNode) -> Result<Constant, AstError>{
        Ok(Constant{
            _type: node["type"].as_str().unwrap().to_string(),
            val: "1".to_string(),
        })
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
    fn simple_main(){
        let ast_root = get_ast("src/operating_system/compiler/test_data/const_expressions/inputs/1.c");
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
                                assert_eq!(c.val, "1");
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