use cpu::instructions::*;
use std::collections::HashMap;
use std::str::FromStr;

fn is_label(line: &str) -> bool {
    line.contains(":")
}

fn get_label_from_line(line: &str) -> Option<String> {
    if is_label(line) {
        return Some(line.trim().replace(":", ""));
    }
    None
}

fn is_instruction(line: &str) -> bool {
    !is_label(line) && line.trim() != ""
}

fn maybe_parse_instruction(
    line: &str,
    symbol_table: &HashMap<String, u32>,
    cur_rel_address: u32,
) -> Option<Instruction> {
    if is_instruction(line) {
        let args: Vec<&str> = line.split_whitespace().collect();
        // if line is flow instruction
        if let Result::Ok(_) = FlowOp::from_str(args[0]) {
            // replace label string with numeric offset
            let label = String::from(args[1]);
            assert!(symbol_table.contains_key(&label));
            let offset = (*symbol_table.get(&label).unwrap() as i32) - (cur_rel_address as i32);
            return Some(Instruction::from_str(&format!("{} {}", args[0], offset)).unwrap());
        }

        if let Ok(instr) = Instruction::from_str(line) {
            return Some(instr);
        }
    }
    None
}

pub fn assemble(program: &str) -> (Vec<Instruction>, HashMap<String, u32>) {
    let mut symbol_table = HashMap::new();
    let mut instructions = Vec::new();
    let mut cur_rel_address = 0;

    // first pass, create symbol table
    let lines: Vec<&str> = program.split("\n").collect();
    for line in lines.iter() {
        if let Some(label) = get_label_from_line(line) {
            symbol_table.insert(label, cur_rel_address);
        } else if is_instruction(line) {
            cur_rel_address += 1;
        }
    }

    // second pass, parse instructions & calc relative offsets
    cur_rel_address = 0;
    let lines: Vec<&str> = program.split("\n").collect();
    for line in lines.iter() {
        if let Some(instr) = maybe_parse_instruction(line, &symbol_table, cur_rel_address) {
            instructions.push(instr);
            cur_rel_address += 1;
        } else if !is_label(line) && line.trim().len() != 0 {
            panic!("Invalid instruction: {}", line);
        }
    }

    (instructions, symbol_table)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_simple_program() {
        let program = "
        MOV R1 3
        ADD R1 R2 R3
        TSTE R1 R2
        PUSH R2
        HALT
        ";
        let (isntructions, symbol_table) = assemble(program);
        if let Instruction::Data {
            ref op,
            ref dst,
            ref src,
        } = isntructions[0]
        {
            assert!(*op == DataOp::MOV);
            assert!(*dst == Register::R1);
            assert!(*src == RegOrImm::Val(3));
        } else {
            panic!();
        }
        if let Instruction::BinArith {
            ref op,
            ref dst,
            ref arg1,
            ref arg2,
        } = isntructions[1]
        {
            assert!(*op == BinArithOp::ADD);
            assert!(*dst == Register::R1);
            assert!(*arg1 == Register::R2);
            assert!(*arg2 == RegOrImm::Reg(Register::R3));
        } else {
            panic!();
        }
        if let Instruction::Test {
            ref op,
            ref arg1,
            ref arg2,
        } = isntructions[2]
        {
            assert!(*op == TestOp::TSTE);
            assert!(*arg1 == Register::R1);
            assert!(*arg2 == RegOrImm::Reg(Register::R2));
        } else {
            panic!();
        }
        if let Instruction::Stack { ref op, ref dst } = isntructions[3] {
            assert!(*op == StackOp::PUSH);
            assert!(*dst == Register::R2);
        } else {
            panic!();
        }
        if let Instruction::Other { ref op } = isntructions[4] {
            assert!(*op == OtherOp::HALT);
        } else {
            panic!();
        }
    }
    #[test]
    fn test_symbol_table() {
        let program = "
        L1:
        MUL R1 R2 5
        JUMP L2
        L3:
        ADD R1 R1 1
        HALT
        L2:
        SUB R2 R2 R1
        TJMP L3
        ";
        let (isntructions, symbol_table) = assemble(program);
        // println!("{:?}", symbol_table);
        assert_eq!(*symbol_table.get("L1").unwrap(), 0);
        assert_eq!(*symbol_table.get("L3").unwrap(), 2);
        assert_eq!(*symbol_table.get("L2").unwrap(), 4);
        if let Instruction::Flow { ref op, ref offset } = isntructions[1] {
            assert_eq!(*op, FlowOp::JUMP);
            assert_eq!(*offset, 3);
        }
        if let Instruction::Flow { ref op, ref offset } = isntructions[5] {
            assert_eq!(*op, FlowOp::TJMP);
            assert_eq!(*offset, -3);
        }
    }
}
