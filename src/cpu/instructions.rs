
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Register{
    R1,
    R2,
    R3,
    R4,
    SP,
    BP,
    IR,
    ZR,
}

use std::str::FromStr;
impl FromStr for Register{
    type Err = ();
    fn from_str(s: &str) -> Result<Register, ()>{
        match s{
            "R1" => Ok(Register::R1),
            "R2" => Ok(Register::R2),
            "R3" => Ok(Register::R3),
            "R4" => Ok(Register::R4),
            "SP" => Ok(Register::SP),
            "BP" => Ok(Register::BP),
            "IR" => Ok(Register::IR),
            "ZR" => Ok(Register::ZR),
            _ => Err(()) ,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinArithOp{
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

impl FromStr for BinArithOp{
    type Err = ();
    fn from_str(s: &str) -> Result<BinArithOp, ()>{
        match s{
            "ADD" => Ok(BinArithOp::ADD),
            "SUB" => Ok(BinArithOp::SUB),
            "MUL" => Ok(BinArithOp::MUL),
            "DIV" => Ok(BinArithOp::DIV),
            "MOD" => Ok(BinArithOp::MOD),
            "AND" => Ok(BinArithOp::AND),
            "OR," => Ok(BinArithOp::OR),
            "SHL" => Ok(BinArithOp::SHL),
            "SHR" => Ok(BinArithOp::SHR),
            "XOR" => Ok(BinArithOp::XOR),
            _ => Err(()) ,
        }
    }
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

#[derive(Debug, PartialEq, Clone)]
pub enum UnaryArithOp{
    NEG,
}
impl FromStr for UnaryArithOp{
    type Err = ();
    fn from_str(s: &str) -> Result<UnaryArithOp, ()>{
        match s{
            "NEG" => Ok(UnaryArithOp::NEG),
            _ => Err(()) , }
    }
}

impl UnaryArithOp{
    fn eval(&self, x: i32) -> i32{
        match &self{
            UnaryArithOp::NEG => -x,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum DataOp{
    LOAD,
    STR,
    MOV,
}

impl FromStr for DataOp{
    type Err = ();
    fn from_str(s: &str) -> Result<DataOp, ()>{
        match s{
            "LOAD" => Ok(DataOp::LOAD),
            "STR" => Ok(DataOp::STR),
            "MOV" => Ok(DataOp::MOV),
            _ => Err(()) ,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum StackOp{
    PUSH,
    POP,
}
impl FromStr for StackOp{
    type Err = ();
    fn from_str(s: &str) -> Result<StackOp, ()>{
        match s{
            "PUSH" => Ok(StackOp::PUSH),
            "POP" => Ok(StackOp::POP),
            _ => Err(()) ,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TestOp{
    TSTE,
    TSTN,
    TSTG,
    TSTL,
}
impl FromStr for TestOp{
    type Err = ();
    fn from_str(s: &str) -> Result<TestOp, ()>{
        match s{
            "TSTE" => Ok(TestOp::TSTE),
            "TSTN" => Ok(TestOp::TSTN),
            "TSTG" => Ok(TestOp::TSTG),
            "TSTL" => Ok(TestOp::TSTL),
            _ => Err(()) ,
        }
    }
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

#[derive(Debug, PartialEq, Clone)]
pub enum FlowOp{
    JUMP,
    TJMP,
    FJMP,
    CALL,
}
impl FromStr for FlowOp{
    type Err = ();
    fn from_str(s: &str) -> Result<FlowOp, ()>{
        match s{
            "JUMP" => Ok(FlowOp::JUMP),
            "TJMP" => Ok(FlowOp::TJMP),
            "FJMP" => Ok(FlowOp::FJMP),
            "CALL" => Ok(FlowOp::CALL),
            _ => Err(()) ,
        }
    }
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

#[derive(Debug, PartialEq, Clone)]
pub enum OtherOp{
    HALT,
    RET,
}
impl FromStr for OtherOp{
    type Err = ();
    fn from_str(s: &str) -> Result<OtherOp, ()>{
        match s{
            "HALT" => Ok(OtherOp::HALT),
            "RET" => Ok(OtherOp::RET),
            _ => Err(()) ,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum RegOrImm{
    Reg(Register),
    Val(i32),
}

// We don't technically need to go with a generic here, but I wanted to expiriment with that
trait HasValue<U>{
    fn evaluate(self) -> U;
}

impl HasValue<RegOrImm> for Register{
    fn evaluate(self) -> RegOrImm{
        RegOrImm::Reg(self)
    }
}

impl HasValue<RegOrImm> for i32{
    fn evaluate(self) -> RegOrImm{
        RegOrImm::Val(self)
    }
}

impl RegOrImm{

    // again, we don't really need this method (because we have from_str), but I wanted to experiment with traits
    fn from<T : HasValue<RegOrImm>>(x : T) -> RegOrImm{
        x.evaluate()
    }

    fn from_str(s: &str) -> Result<RegOrImm, ()>{
        if let Ok(reg) = Register::from_str(s){
            Ok(RegOrImm::from(reg))
        }else{
            if let Ok(x) = s.parse::<i32>(){
                Ok(RegOrImm::from(x))
            }else{
                Err(())
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Instruction{
    UnaryArith {op: UnaryArithOp, arg: Register},
    BinArith {op : BinArithOp, dst: Register, arg1: Register, arg2: RegOrImm},
    Data {op: DataOp, dst: Register, src: RegOrImm},
    Stack {op: StackOp, dst: Register},
    Test {op: TestOp, arg1: Register, arg2: RegOrImm},
    Flow {op: FlowOp, offset: i32},
    Other {op: OtherOp},
}

impl Instruction{
    pub fn to_str(&self) -> String{
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

    pub fn from_str(instruction_str : &str) -> Result<Instruction, ()>{
        let args : Vec<&str> = instruction_str.split_whitespace().collect();
        println!("{:?}", args);
        let op = args[0];
        if let Result::Ok(op) = UnaryArithOp::from_str(&op){
            assert!(args.len() == 2);
            return Ok(Instruction::UnaryArith {
                op: op,
                arg: Register::from_str(args[1]).unwrap(),
            })
        } else if let Result::Ok(op) = BinArithOp::from_str(&op){
            assert!(args.len() == 4);
            return Ok(Instruction::BinArith{
                op: op,
                dst: Register::from_str(args[1]).unwrap(),
                arg1: Register::from_str(args[2]).unwrap(),
                arg2: RegOrImm::from_str(args[3]).unwrap(),
            })
        } else if let Result::Ok(op) = DataOp::from_str(&op){
            assert!(args.len() == 3);
            return Ok(Instruction::Data{
                op: op,
                dst: Register::from_str(args[1]).unwrap(),
                src: RegOrImm::from_str(args[2]).unwrap(),
            })
        } else if let Result::Ok(op) = StackOp::from_str(&op){
            assert!(args.len() == 2);
            return Ok(Instruction::Stack{
                op: op,
                dst: Register::from_str(args[1]).unwrap()
            })
        } else if let Result::Ok(op) = TestOp::from_str(&op){
            assert!(args.len() == 3);
            return Ok(Instruction::Test{
                op: op,
                arg1: Register::from_str(args[1]).unwrap(),
                arg2: RegOrImm::from_str(args[2]).unwrap(),
            })
        } else if let Result::Ok(op) = FlowOp::from_str(&op){
            assert!(args.len() == 2);
            return Ok(Instruction::Flow{
                op: op,
                offset: args[1].parse::<i32>().unwrap()
            })
        } else if let Result::Ok(op) = OtherOp::from_str(&op){
            assert!(args.len() == 1);
            return Ok(Instruction::Other{
                op: op
            })
        }
        Err(())
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn neg_from_str(){
        assert_eq!(Instruction::from_str("NEG R1").unwrap(), Instruction::UnaryArith{op: UnaryArithOp::NEG, arg: Register::R1});
    }
    #[test]
    fn mul_from_str_reg(){
        assert_eq!(Instruction::from_str("MUL R1 R1 R2").unwrap(), Instruction::BinArith{op: BinArithOp::MUL, dst: Register::R1, arg1: Register::R1, arg2: RegOrImm::Reg(Register::R2)})
    }
    #[test]
    fn mul_from_str_imm(){
        assert_eq!(Instruction::from_str("MUL R1 R1 3").unwrap(), Instruction::BinArith{op: BinArithOp::MUL, dst: Register::R1, arg1: Register::R1, arg2: RegOrImm::Val(3)})
    }
    #[test]
    fn  mov_from_str_reg(){
        assert_eq!(Instruction::from_str("MOV R1 R2").unwrap(), Instruction::Data{op: DataOp::MOV, dst: Register::R1, src:RegOrImm::Reg(Register::R2)})
    }
    #[test]
    fn  mov_from_str_imm(){
        assert_eq!(Instruction::from_str("MOV R1 3").unwrap(), Instruction::Data{op: DataOp::MOV, dst: Register::R1, src:RegOrImm::Val(3)})
    }
    #[test]
    fn  push_from_str(){
        assert_eq!(Instruction::from_str("PUSH R1").unwrap(), Instruction::Stack{op: StackOp::PUSH, dst: Register::R1})
    }
    #[test]
    fn  tstg_from_str_reg(){
        assert_eq!(Instruction::from_str("TSTG R1 R2").unwrap(), Instruction::Test{op: TestOp::TSTG, arg1:Register::R1, arg2:RegOrImm::Reg(Register::R2)})
    }
    #[test]
    fn  tstg_from_str_imm(){
        assert_eq!(Instruction::from_str("TSTG R1 3").unwrap(), Instruction::Test{op: TestOp::TSTG, arg1:Register::R1, arg2:RegOrImm::Val(3)})
    }
    #[test]
    fn  tjmp_from_str(){
        assert_eq!(Instruction::from_str("TJMP 10").unwrap(), Instruction::Flow{op: FlowOp::TJMP, offset: 10})
    }
    #[test]
    fn  halt_from_str(){
        assert_eq!(Instruction::from_str("HALT").unwrap(), Instruction::Other{op: OtherOp::HALT})
    }
    #[test]
    fn  ret_from_str(){
        assert_eq!(Instruction::from_str("RET").unwrap(), Instruction::Other{op: OtherOp::RET})
    }
}