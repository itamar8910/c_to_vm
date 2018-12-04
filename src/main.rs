mod cpu;
mod operating_system;

use std::env;
use operating_system::compiler::Compiler;
use operating_system::OS;

fn main() {
    let args : Vec<String> = env::args().collect();
    println!("compiling: {}", args[1]);
    let program = Compiler::compile(&args[1]);
    println!("finished compiling");
    let mut os = OS::new();
    let res = os.assemble_and_run(&program);
    println!("{}", program);
    println!("-----");
    println!("Return code:{}", res);
}
