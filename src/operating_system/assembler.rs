use std::str::FromStr;
use std::collections::HashMap;
use ::cpu::instructions::*;

fn is_label(line: &str) -> bool{
    line.contains(":")
}

fn get_label_from_line(line: &str) -> Option<String>{
    if is_label(line){
        return Some(line.trim().replace(":",""));
    }
    None
}

fn is_instruction(line: &str) -> bool{
    !is_label(line) && line.trim() != ""
}

fn maybe_parse_instruction(line: &str, symbol_table: &HashMap<String, u32>, cur_rel_address: u32) -> Option<Instruction>{
    if is_instruction(line){
        let args : Vec<&str> = line.split_whitespace().collect();
        // if line is flow instruction
        if let Result::Ok(flow_op) = FlowOp::from_str(args[0]){
            // replace label string with numeric offset
            let label = get_label_from_line(line).unwrap();
            assert!(symbol_table.contains_key(&label));
            let offset = symbol_table.get(&label).unwrap() - cur_rel_address;
            return Some(Instruction::from_str(&format!("{} {}", args[0], offset )).unwrap());

        }
        
        if let Ok(instr) = Instruction::from_str(line){
            return Some(instr);
        }
    }
    None
}

fn assemble(program: &str) -> (Vec<Instruction>, HashMap<String, u32>){
    let mut symbol_table = HashMap::new();
    let mut instructions = Vec::new();
    let mut cur_rel_address = 0; 

    // first pass, create symbol table
    let lines: Vec<&str> = program.split("\n").collect();
    for line in lines.iter(){
       if let Some(label) = get_label_from_line(line){
           symbol_table.insert(label, cur_rel_address);
       } else if is_instruction(line){
           cur_rel_address += 1;
       }
    }

    // second pass, parse instructions & calc relative offsets
    cur_rel_address = 0;
    let lines: Vec<&str> = program.split("\n").collect();
    for line in lines.iter(){
        if let Some(instr) = maybe_parse_instruction(line, &symbol_table, cur_rel_address){
            instructions.push(instr);
            cur_rel_address += 1;
        }
    } 

    (instructions, symbol_table)
}


#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_simple_program(){
        let program = "
        MOV R1 3
        ADD R1 R2 R3
        TSTE R1 R2
        PUSH R2
        HALT
        ";
        let (isntructions, symbol_table) = assemble(program);
        if let Instruction::Data {ref op, ref dst, ref src} = isntructions[0]{
            assert!(*op == DataOp::MOV);
            assert!(*dst == Register::R1);
            assert!(*src == RegOrImm::Val(3));
        } else{
            panic!();
        }
        if let Instruction::BinArith {ref op, ref dst, ref arg1, ref arg2} = isntructions[1]{
            assert!(*op == BinArithOp::ADD);
            assert!(*dst == Register::R1);
            assert!(*arg1 == Register::R2);
            assert!(*arg2 == RegOrImm::Reg(Register::R3));
        }else{
            panic!();
        }
        if let Instruction::Test {ref op, ref arg1, ref arg2} = isntructions[2]{
            assert!(*op == TestOp::TSTE);
            assert!(*arg1 == Register::R1);
            assert!(*arg2 == RegOrImm::Reg(Register::R2));
        } else{
            panic!();
        }
        if let Instruction::Stack {ref op, ref dst} = isntructions[3]{
            assert!(*op == StackOp::PUSH);
            assert!(*dst == Register::R2);
        } else{
            panic!();
        }
        if let Instruction::Other {ref op} = isntructions[4]{
            assert!(*op == OtherOp::HALT);
        } else{
            panic!();
        }
    }
}