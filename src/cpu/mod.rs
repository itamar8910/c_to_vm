pub mod instructions;

use std::collections::HashMap;
use self::instructions::*;

struct Registers{
    values: HashMap<Register, i32>,
}

impl Registers{
    fn new() -> Registers{
        let mut instance = Registers{values: HashMap::new()};
        instance.values.insert(Register::R1, 0);
        instance.values.insert(Register::R2, 0);
        instance.values.insert(Register::R3, 0);
        instance.values.insert(Register::R4, 0);
        instance.values.insert(Register::IR, 0);
        instance.values.insert(Register::SP, 0);
        instance.values.insert(Register::BP, 0);
        instance.values.insert(Register::ZR, 0);
        instance
    } 
    fn get(&self, reg : Register) -> i32{
        *self.values.get(&reg).unwrap()
    }
    fn set(&mut self, reg: Register, val: i32){
        self.values.insert(reg, val);
    }
}

enum MemEntry{
    num(i32),
    instruciton(Instruction),
}

struct Memory{
    data: HashMap<u32, MemEntry>
}
impl Memory{
    fn new() -> Memory{
        Memory{data:HashMap::new()}
    }
    fn get(&self, address: u32) -> &MemEntry{
        self.data.get(&address).expect("Invalid memory access")
    }
    fn set(&mut self, address: u32, val: MemEntry){
        self.data.insert(address, val);
    }
}

struct Cpu{
    mem: Memory,
    regs: Registers,
}

impl Cpu{
    fn new() -> Cpu{
        Cpu{mem: Memory::new(), regs: Registers::new()}
    }

    fn fetch(&self) -> Instruction{
        if let MemEntry::instruciton(instr) = self.mem.get(self.regs.get(Register::IR) as u32){
            return instr.clone()
        }
        panic!("cannot execute data!");
    }
    fn execute_unary_arith(&mut self, op: &UnaryArithOp, arg: &Register){

    }
    fn execute_bin_arith(&mut self, op: &BinArithOp, dst: &Register, arg1: &Register, arg2: &RegOrImm){

    }
    fn execute_data(&mut self, op: &DataOp, dst: &Register, src: &RegOrImm){

    }
    fn execute_stack(&mut self, op: &StackOp, dst: &Register){

    }
    fn execute_test(&mut self, op: &TestOp, arg1: &Register, arg2: &RegOrImm){

    }
    fn execute_flow(&mut self, op: &FlowOp, offset: i32){

    }
    fn execute_other(&mut self, op: &OtherOp){

    }
    /**
     * executes instruction
     * returns whether CPU should keep running
     */
    fn execute(&mut self, instr: &Instruction) -> bool{
        match instr{
            Instruction::UnaryArith {op, arg} => {
                self.execute_unary_arith(op, arg);
                return true;
            },
            Instruction::BinArith {op, dst, arg1, arg2} => {
                self.execute_bin_arith(op, dst, arg1, arg2);
                return true
            },
            Instruction::Data {op, dst, src} => {
                self.execute_data(op, dst, src);
                return true
            },
            Instruction::Stack {op, dst} => {
                self.execute_stack(op, dst);
                return true
            },
            Instruction::Test {op, arg1, arg2} => {
                self.execute_test(op, arg1, arg2);
                return true
            },
            Instruction::Flow {op, offset} => {
                self.execute_flow(op, *offset);
                return true
            },
            Instruction::Other {op} => {
                self.execute_other(op);
                return true
            },
        }
    }

    fn start(&mut self){
        loop{
            let instr = self.fetch();
            let keep_running = self.execute(&instr);
            if !keep_running{
                break;
            }

        }
    }
}