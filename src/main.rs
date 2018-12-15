#[macro_use] extern crate matches;
mod cpu;
mod operating_system;

use crate::operating_system::compiler::Compiler;
use crate::operating_system::OS;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3{
        panic!("Usage: [run|debug] path_to_c_file/s")
    }
    let mut programs = Vec::new();
    for program_i in 2..args.len(){
        println!("compiling: {}", args[program_i]);
        let program = Compiler::compile(&args[program_i]);
        let lines: Vec<&str> = program.split("\n").collect();
        for (line_i, line) in lines.iter().enumerate(){
            println!("{}: {}", line_i, line);
        }
        programs.push(program);
    }
    let programs = programs.iter().map(|s| s.as_str()).collect();
    let mut os = OS::new();
    let mut res = -1;
    if args[1] == "run"{
        res = os.assemble_link_and_run(programs);
    } else if args[1] == "debug"{
        res = os.assemble_and_debug(programs);
    }else{
        panic!("invalid run mode")
    }
    println!("\n--------");
    println!("Return code:{}", res);
}
