
struct RootAstNode{
    externals: Vec<External>,
}

// TODO: implement Deserialize from JSON
// we could use serde's Deserialize trait
// but it will probably only burden us because we won't use any of its automatic deserialization features
// Anyways, use this for reference: https://users.rust-lang.org/t/serde-deserialize-custom-implementation/17124
enum External{
    FuncDef(FuncDef),
}

struct FuncDef{
    body: Compound,
    decl: FuncDecl,
}

struct FuncDecl{
    name: String,
    argsType: Vec<String>,
    retType: String,
}

struct Compound{
    items: Vec<Statement>,
}

enum Statement{
    Return(Return),
}
struct Return{
    expr: Expression,
}
enum Expression{
    Constant(Constant),
}

struct Constant{
    _type: String,
    val: i32,
}


// pub fn get_ast(path_to_s_source: &str) -> RootASTNode{

// }