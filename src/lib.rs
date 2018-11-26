pub mod cpu;
pub mod operating_system;

use operating_system::assembler::assemble;
use operating_system::OS;

fn assemble_and_run(program: &str) -> i32{
    let mut os = OS::new();
    let (instructions, _) = assemble(program);
    os.run_program(instructions)
}