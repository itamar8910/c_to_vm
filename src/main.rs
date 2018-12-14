mod cpu;
mod operating_system;

use crate::operating_system::compiler::Compiler;
use crate::operating_system::OS;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3{
        panic!("Usage: [run|debug] path_to_c_file")
    }
    println!("compiling: {}", args[2]);
    let program = Compiler::compile(&args[2]);
    let lines: Vec<&str> = program.split("\n").collect();
    for (line_i, line) in lines.iter().enumerate(){
        println!("{}: {}", line_i, line);
    }
    println!("RUNNING");
    println!("--------");
    let mut os = OS::new();
    let mut res = -1;
    if args[1] == "run"{
        res = os.assemble_and_run(&program);
    } else if args[1] == "debug"{
        res = os.assemble_and_debug(&program);
    }else{
        panic!("invalid run mode")
    }
    println!("\n--------");
    println!("Return code:{}", res);
}
