use crate::cpu::instructions::*;
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
            assert!(symbol_table.contains_key(&label), format!("label:{} does not exist in symbol table", label));
            let offset = (*symbol_table.get(&label).unwrap() as i32) - (cur_rel_address as i32);
            return Some(Instruction::from_str(&format!("{} {}", args[0], offset)).unwrap());
        }

        if let Ok(instr) = Instruction::from_str(line) {
            return Some(instr);
        }
    }
    None
}


/// returns program's symbol table & # of instructions
pub fn gen_symbol_table(program: &str, start_addr: u32) -> (HashMap<String, u32>, u32){
    let mut symbol_table = HashMap::new();
    let mut cur_address = start_addr;

    let lines: Vec<&str> = program.split("\n").collect();
    for (line_i, line) in lines.iter().enumerate() {
        if let Some(label) = get_label_from_line(line) {
            symbol_table.insert(label, cur_address);
        } else if is_instruction(line) {
            cur_address += 1;
        }
    }

    (symbol_table, cur_address - start_addr)
}

pub fn assemble(program: &str) -> (Vec<Instruction>, HashMap<String, u32>) {
    assemble_and_link(vec![program])
}

pub fn assemble_and_link(programs: Vec<&str>) -> (Vec<Instruction>, HashMap<String, u32>) {
    let mut symbol_table = HashMap::new();
    let mut instructions = Vec::new();
    let mut cur_rel_address = 0;

    // create a symbol table for each program separately 
    // and add it to global symbol table
    // side note: we create a separate symobl table for each file instead of just concatenating all of the programs
    // in order to be able to support source-level breakpoints in the future
    for program in programs.iter(){
        let (program_symbol_table, program_size) = gen_symbol_table(*program, cur_rel_address);
        cur_rel_address += program_size;
        symbol_table.extend(program_symbol_table);
    }
    let whole_program = programs.join("\n");
    println!("--------");
    for (line_i, line) in whole_program.split("\n").collect::<Vec<&str>>().iter().enumerate(){
        println!("{}: {}", line_i, line);
    }
    println!("--------");
    // second pass, parse instructions & calc relative offsets
    cur_rel_address = 0;
    let lines: Vec<&str> = whole_program.split("\n").collect();
    for (line_i, line) in lines.iter().enumerate() {
        symbol_table.insert(format!("_LINE_{}", line_i.to_string()), cur_rel_address); // for setting breakpoints in debugger
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
        let (isntructions, _symbol_table) = assemble(program);
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
