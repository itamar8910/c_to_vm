mod cpu;
mod operating_system;

fn main() {
    println!("Hello, world!");
    let json = operating_system::compiler::get_ast_json("example.c");
    println!("{}", json);
    // let instructions = operating_system::compiler::Compiler::compile("example.c");
    // println!("{:?}", instructions);
    // println!("{}",cpu::instructions::Register::R2);
}
