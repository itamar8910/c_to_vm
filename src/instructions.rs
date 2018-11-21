
#[derive(Debug)]
enum Register{
    R1,
    R2,
    R3,
    R4,
    SP,
    BP,
    IR,
    ZR,
}

#[derive(Debug)]
enum BinArithOp{
    ADD,
    SUB,
    MUL,
    DIV,
    MOD,
    AND,
    OR,
    SHL,
    SHR,
    XOR,
}

impl BinArithOp{
    fn eval(&self, x: i32, y: i32) -> i32{
        match &self{
            BinArithOp::ADD => x + y,
            BinArithOp::SUB => x - y,
            BinArithOp::MUL => x * y,
            BinArithOp::DIV => x / y,
            BinArithOp::MOD => x % y,
            BinArithOp::AND => x & y,
            BinArithOp::OR => x | y,
            BinArithOp::SHL => x << y,
            BinArithOp::SHR => x >> y,
            BinArithOp::XOR => x ^ y,
        }
    }
}

#[derive(Debug)]
enum UnaryArithOp{
    NEG,
}

impl UnaryArithOp{
    fn eval(&self, x: i32) -> i32{
        match &self{
            UnaryArithOp::NEG => -x,
        }
    }
}

#[derive(Debug)]
enum DataOp{
    LOAD,
    STR,
    MOV,
}

#[derive(Debug)]
enum StackOp{
    PUSH,
    POP,
}

#[derive(Debug)]
enum TestOp{
    TSTE,
    TSTN,
    TSTG,
    TSTL,
}

impl TestOp{
    fn test(&self, arg1 : i32, arg2: i32) -> bool{
        match &self{
            TestOp::TSTE => arg1 == arg2,
            TestOp::TSTN => arg1 != arg2,
            TestOp::TSTG => arg1 > arg2,
            TestOp::TSTL => arg1 < arg2,
        }
    }
}

#[derive(Debug)]
enum FlowOp{
    JUMP,
    TJMP,
    FJMP,
    CALL,
}

impl FlowOp{
    fn should_take(&self, arg : i32) -> bool{
        match &self{
            FlowOp::JUMP => true,
            FlowOp::TJMP => arg != 0,
            FlowOp::FJMP => arg == 0,
            FlowOp::CALL => true,
        }

    }
}

#[derive(Debug)]
enum OtherOp{
    HALT,
    RET,
}

#[derive(Debug)]
enum RegOrImm{
    Reg(Register),
    Val(i32),
}

#[derive(Debug)]
enum Instruction{
    UnaryArith {op: UnaryArithOp, arg: Register},
    BinArith {op : BinArithOp, dst: Register, arg1: Register, arg2: RegOrImm},
    Data {op: DataOp, dst: Register, src: RegOrImm},
    Stack {op: StackOp, dst: Register},
    Test {op: TestOp, arg1: Register, arg2: RegOrImm},
    Flow {op: FlowOp, offset: i32},
    Other {op: OtherOp},
}

impl Instruction{
    fn to_str(&self) -> String{
        match &self{
            Instruction::UnaryArith{op, arg} => format!("{:?} {:?}", op, arg),
            Instruction::BinArith{op, dst, arg1, arg2} => format!("{:?} {:?} {:?} {:?}", op,  dst, arg1, arg2),
            Instruction::Data{op, dst, src} => format!("{:?} {:?} {:?}", op, dst, src),
            Instruction::Stack{op, dst} => format!("{:?} {:?}", op, dst),
            Instruction::Test{op, arg1, arg2} => format!("{:?} {:?} {:?}", op, arg1, arg2),
            Instruction::Flow{op, offset} => format!("{:?} {:?}", op, offset),
            Instruction::Other{op} => format!("{:?}", op)
        }
    }

    fn from_str(instruction_str : &str) -> Instruction{
        let args = instruction_str.split("").collect::<Vec<&str>>();
        // TODO
        // just so this will compile
        Instruction::BinArith{
            op: BinArithOp::ADD,
            dst: Register::R1,
            arg1: Register::R1,
            arg2: RegOrImm::Reg(Register::R2),  // TODO: refactor to RegOrImm::from method
        }
        
    }

}