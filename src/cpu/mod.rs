pub mod instructions;

use self::instructions::*;
use std::collections::HashMap;

pub struct Registers {
    values: HashMap<Register, i32>,
}

impl Registers {
    fn new() -> Registers {
        let mut instance = Registers {
            values: HashMap::new(),
        };
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
    pub fn get(&self, reg: &Register) -> i32 {
        *self.values.get(&reg).unwrap()
    }
    pub fn set(&mut self, reg: &Register, val: i32) {
        self.values.insert(reg.clone(), val);
    }
    pub fn get_reg_or_imm(&self, arg: &RegOrImm) -> i32 {
        match arg {
            RegOrImm::Reg(reg) => {
                return self.get(reg);
            }
            RegOrImm::Val(val) => {
                return *val;
            }
        };
    }
}

pub enum MemEntry {
    Num(i32),
    Instruction(Instruction),
}

pub struct Memory {
    data: HashMap<u32, MemEntry>,
}
impl Memory {
    fn new() -> Memory {
        Memory {
            data: HashMap::new(),
        }
    }
    pub fn get(&self, address: u32) -> &MemEntry {
        self.data
            .get(&address)
            .expect(format!("Invalid memory access: {}", address).as_str())
    }
    pub fn set(&mut self, address: u32, val: MemEntry) {
        self.data.insert(address, val);
    }
    pub fn get_num(&self, address: u32) -> i32 {
        match self.get(address) {
            MemEntry::Num(x) => *x,
            MemEntry::Instruction(_) => panic!("not numeric value"),
        }
    }
}

pub struct Cpu {
    pub mem: Memory,
    pub regs: Registers,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            mem: Memory::new(),
            regs: Registers::new(),
        }
    }

    fn fetch(&self) -> Instruction {
        if let MemEntry::Instruction(instr) = self.mem.get(self.regs.get(&Register::IR) as u32) {
            return instr.clone();
        }
        panic!("cannot execute data!");
    }
    fn execute_unary_arith(&mut self, op: &UnaryArithOp, arg: &Register) {
        let reg_val = self.regs.get(arg);
        let res = op.eval(reg_val);
        self.regs.set(arg, res);
    }
    fn execute_bin_arith(
        &mut self,
        op: &BinArithOp,
        dst: &Register,
        arg1: &Register,
        arg2: &RegOrImm,
    ) {
        let arg1_val = self.regs.get(arg1);
        let arg2_val = self.regs.get_reg_or_imm(arg2);
        let res = op.eval(arg1_val, arg2_val);
        self.regs.set(dst, res);
    }
    fn execute_data(&mut self, op: &DataOp, dst: &Register, src: &RegOrImm) {
        let src_val = self.regs.get_reg_or_imm(src);
        match op {
            DataOp::LOAD => {
                let mem_src_val = self.mem.get_num(src_val as u32);
                self.regs.set(dst, mem_src_val);
            }
            DataOp::STR => {
                self.mem
                    .set(self.regs.get(dst) as u32, MemEntry::Num(src_val));
            }
            DataOp::MOV => {
                self.regs.set(dst, src_val);
            }
        }
    }
    fn execute_stack(&mut self, op: &StackOp, dst: &Register) {
        let sp = self.regs.get(&Register::SP);
        match op {
            StackOp::PUSH => {
                let dst_val = self.regs.get(dst);
                self.mem.set(sp as u32, MemEntry::Num(dst_val));
                self.regs.set(&Register::SP, sp - 1);
            }
            StackOp::POP => {
                self.regs.set(dst, self.mem.get_num(sp as u32 + 1));
                self.regs.set(&Register::SP, sp + 1);
            }
        }
    }
    fn execute_test(&mut self, op: &TestOp, arg1: &Register, arg2: &RegOrImm) {
        let arg1_val = self.regs.get(arg1);
        let arg2_val = self.regs.get_reg_or_imm(arg2);
        let res = op.test(arg1_val, arg2_val);
        self.regs.set(&Register::ZR, if res { 1 } else { 0 });
    }

    fn execute_flow(&mut self, op: &FlowOp, offset: i32) {
        if op.should_take(self.regs.get(&Register::ZR)) {
            if let FlowOp::CALL = op {
                let sp = self.regs.get(&Register::SP);
                // push ret address
                self.mem
                    .set(sp as u32, MemEntry::Num(self.regs.get(&Register::IR) + 1));
                // push caller BP
                self.mem
                    .set(sp as u32 - 1, MemEntry::Num(self.regs.get(&Register::BP)));
                self.regs.set(&Register::BP, sp - 1);
                self.regs.set(&Register::SP, sp - 2);
            }
            let ir = self.regs.get(&Register::IR);
            self.regs.set(&Register::IR, ir + offset - 1);
        }
    }
    fn execute_other(&mut self, op: &OtherOp) {
        match op {
            OtherOp::HALT => {}
            OtherOp::RET => {
                let bp = self.regs.get(&Register::BP);
                self.regs.set(&Register::SP, bp + 1);
                let ret_addr = self.mem.get_num(bp as u32 + 1);
                self.regs.set(&Register::BP, self.mem.get_num(bp as u32));
                self.regs.set(&Register::IR, ret_addr - 1); // IR will be increment at end of cycle
            }
        }
    }
    /**
     * executes instruction
     * returns whether CPU should keep running
     */
    fn execute(&mut self, instr: &Instruction) -> bool {
        match instr {
            Instruction::UnaryArith { op, arg } => {
                self.execute_unary_arith(op, arg);
                return true;
            }
            Instruction::BinArith {
                op,
                dst,
                arg1,
                arg2,
            } => {
                self.execute_bin_arith(op, dst, arg1, arg2);
                return true;
            }
            Instruction::Data { op, dst, src } => {
                self.execute_data(op, dst, src);
                return true;
            }
            Instruction::Stack { op, dst } => {
                self.execute_stack(op, dst);
                return true;
            }
            Instruction::Test { op, arg1, arg2 } => {
                self.execute_test(op, arg1, arg2);
                return true;
            }
            Instruction::Flow { op, offset } => {
                self.execute_flow(op, *offset);
                return true;
            }
            Instruction::Other { op } => {
                self.execute_other(op);
                return if let OtherOp::HALT = op { false } else { true };
            }
        }
    }

    pub fn start(&mut self) {
        loop {
            println!(
                "fetching instruction from: {}",
                self.regs.get(&Register::IR)
            );
            let instr = self.fetch();
            let keep_running = self.execute(&instr);
            let ir = self.regs.get(&Register::IR);
            self.regs.set(&Register::IR, ir + 1);
            if !keep_running {
                break;
            }
        }
    }
}
