pub mod assembler;
pub mod compiler;

use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Read;

use self::assembler::assemble;
use self::assembler::assemble_and_link;
use self::compiler::Compiler;
use crate::cpu::instructions::*;
use crate::cpu::Cpu;
use crate::cpu::MemEntry;

/*
Memory layout:
0-499 os stuff:
    - memory mapped registers:
    - 200 COS - char out status
    - 201 COD - char out data
    - 202 CIS - char in status
    - 203 CID - char in data
    
    to write a char, write its ascii value to COD & then set COS to 1
    to read a char, set CIS to 1 & read ascii value from CID
500-999 data
1000-3999 code
4000-5999 heap
6000-9999 stack


Stack frame:
local vars...
-----------------
reg_save (callee save)
----------------
prev_BP
ret_addr
ret_val (can span multiple addresses)
--------------
arg1
arg2
arg3


Call convention:
Calling the function:
    Caller: 
        - pushes args on the stack in reverse order
        - pushes space for return value (callee does this because distance between BP & ret val must be constant for RET instructions)
        - CALL - pushes return address (= IP + 1),
                 pushes value of current bp & updates bp=sp+1
                 jumps to function
    Callee:
        - 
        - saves all registers whose value would get destroyed
        - can allocate local vars on the stack etc.
Returning from the function:
    Callee:
        - pushes return value to the stack
        - restores values of saved registers
        - 
        - RET - SP = BP + 1
                restores BP
                jump to returna addr
*/

const PROGRAM_INIT_ADDRESS: u32 = 1000;
const INIT_SP_ADDRESS: u32 = 9999;

// memory mapped registers for io
const COS : u32 = 200; // char out status
const COD : u32 = 201; // char out data
const CIS : u32 = 202; // char in status
const CID : u32 = 203; // char in data

pub struct OS {
    pub cpu: Cpu,
    pub out_chars : Vec<char>,
    pub inp_chars : Vec<char>,
    std_programs: Vec<String>,
}

impl OS {
    pub fn new() -> OS {
        let mut std_programs = Vec::new();
        std_programs.push(Compiler::compile("libc/libc.c"));
        let mut instance = OS { cpu: Cpu::new() , out_chars: Vec::new(), inp_chars: Vec::new(),
            std_programs};
        instance.initialize_memory();
        instance
    }

    fn initialize_memory(&mut self) {
        self.cpu.mem.set(
            0,
            MemEntry::Instruction(Instruction::from_str("HALT").unwrap()),
        );
        self.cpu.mem.set(COS, MemEntry::Num(0));
        self.cpu.mem.set(COD, MemEntry::Num(0));
        self.cpu.mem.set(CIS, MemEntry::Num(0));
        self.cpu.mem.set(CID, MemEntry::Num(0));
    }

    fn reset_cpu_state(&mut self) {
        self.cpu = Cpu::new();
        self.initialize_memory();
    }

    fn initialize_stackframe(&mut self) {
        self.cpu
            .regs
            .set(&Register::SP, (INIT_SP_ADDRESS - 3) as i32);
        self.cpu
            .regs
            .set(&Register::BP, (INIT_SP_ADDRESS - 2) as i32);

        self.cpu.mem.set(INIT_SP_ADDRESS - 1, MemEntry::Num(0)); // jump to HALT in the end
        self.cpu.mem.set(
            INIT_SP_ADDRESS - 2,
            MemEntry::Num((INIT_SP_ADDRESS - 2) as i32),
        ); // no prev BP, BP points to itself
        self.cpu.mem.set(INIT_SP_ADDRESS, MemEntry::Num(-1)); // deafult return value = -1
    }

    fn load_program(&mut self, instructions: &Vec<Instruction>, init_addr: u32) {
        for (instr_i, instr) in instructions.iter().enumerate() {
            self.cpu.mem.set(
                init_addr + (instr_i as u32),
                MemEntry::Instruction(instr.clone()),
            );
        }
    }

    fn io_step(&mut self){
        if self.cpu.mem.get_num(COS) != 0 {
            let c = self.cpu.mem.get_num(COD);
            let c = c as u8 as char;
            self.out_chars.push(c);
            print!("{}", c);
            // reset status register
            self.cpu.mem.set(COS, MemEntry::Num(0));
        }
        if self.cpu.mem.get_num(CIS) != 0 {
            // read a single byte fron stdin
            let mut input_handle = std::io::stdin().take(1);
            let mut buffer = [0];
            input_handle.read(&mut buffer);
            let c = buffer[0] as char;
            self.cpu.mem.set(CID, MemEntry::Num(c as i32));
            self.cpu.mem.set(CIS, MemEntry::Num(0));
        }
    }

    fn step(&mut self) -> bool {
        let keep_running = self.cpu.step();
        self.io_step();
        keep_running
    }

    fn run(&mut self){
        loop{
            let keep_running = self.step();
            if !keep_running {
                break;
            }
        }
    }

    // runs given program
    // returns program's exit value
    pub fn load_and_run(&mut self, instructions: Vec<Instruction>) -> i32 {
        self.reset_cpu_state();
        self.load_program(&instructions, PROGRAM_INIT_ADDRESS);
        self.cpu
            .regs
            .set(&Register::IR, PROGRAM_INIT_ADDRESS as i32);
        self.initialize_stackframe();
        self.run();

        let bp = self.cpu.regs.get(&Register::BP);
        self.cpu.mem.get_num((bp + 2) as u32)
    }

    pub fn assemble_link_and_run(&mut self, programs: Vec<&str>) -> i32 {
        let mut programs_with_std = programs;
        let mut std_programs_clone = self.std_programs.iter().map(|s| s.as_str()).collect();
        programs_with_std.append(&mut std_programs_clone);
        let (instructions, _) = assemble_and_link(programs_with_std);
        self.load_and_run(instructions)
    }

    pub fn assemble_and_run(&mut self, program: &str) -> i32 {
        self.assemble_link_and_run(vec![program])
    }

    pub fn assemble_and_run_no_std(&mut self, program: &str) -> i32{
        let (instructions, _) = assemble_and_link(vec![program]);
        self.load_and_run(instructions)
    }

    pub fn debug_program(&mut self, instructions: Vec<Instruction>, symbol_table: HashMap<String, u32>) -> i32{
        self.reset_cpu_state();
        self.load_program(&instructions, PROGRAM_INIT_ADDRESS);
        self.cpu
            .regs
            .set(&Register::IR, PROGRAM_INIT_ADDRESS as i32);
        self.initialize_stackframe();
        let mut breakpoints : HashSet<u32> = HashSet::new();
        let mut running = false;
        let mut keep_running = true;
        while keep_running{
            let cur_instr_addr = self.cpu.regs.get(&Register::IR);
            // println!("{}: {}", cur_instr_addr - PROGRAM_INIT_ADDRESS as i32, self.cpu.fetch().to_str());
            if breakpoints.contains(&(cur_instr_addr as u32 - PROGRAM_INIT_ADDRESS)){
                running = false;
            }
            if running{
                keep_running = self.step();
                continue;
            }
            let next_instr = self.cpu.fetch();
            println!("{}: {}", self.cpu.regs.get(&Register::IR) - PROGRAM_INIT_ADDRESS as i32, next_instr.to_str());
            use std::io::{stdin,stdout,Write};
            let mut cmd = String::new();
            if let Some('\n')=cmd.chars().next_back() {
                cmd.pop();
            }
            stdin().read_line(&mut cmd).expect("");
            let args: Vec<&str> = cmd.split_whitespace().collect();
            if args.len() == 0{
                continue;
            }
            if args[0] == "continue"{
                running = true;
            }
            if args[0] == "step"{
                keep_running = self.cpu.step();
            }
            if args[0] == "reg"{
                let reg = register_from_str(args[1]).unwrap();
                let reg_val = self.cpu.regs.get(&reg);
                println!("{}", reg_val);
            }
            if args[0] == "break"{
                let line = args[1];
                let instr_i = symbol_table.get(&format!("_LINE_{}", line)).expect("invalid breakpoint line");
                println!("break instr: {:?}", &instructions[*instr_i as usize]);
                breakpoints.insert(*instr_i);

            }
            
        }

        let bp = self.cpu.regs.get(&Register::BP);
        self.cpu.mem.get_num((bp + 2) as u32)
    }

    pub fn assemble_and_debug(&mut self, programs: Vec<&str>) -> i32 {
        let (instructions, symbol_table) = assemble_and_link(programs);
        self.debug_program(instructions, symbol_table)
    }

}
