use crate::cpu::instructions::*;
use super::layout::DATA_INIT_ADDRESS;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::hash_set::Intersection;
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
    data_table: &HashMap<String, u32>,
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
        if let Result::Ok(lea) = DataOp::from_str(args[0]){
            if matches!(lea, DataOp::LEA) {
                let dst = String::from(args[1]);
                let label = String::from(args[2]);
                assert!(data_table.contains_key(&label), format!("label:{} does not exist in data table", label));
                let label_addr = data_table.get(&label).unwrap() + DATA_INIT_ADDRESS;
                return Some(Instruction::from_str(&format!("LEA {} {}", dst, label_addr)).unwrap());
            }
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

fn is_data(line: &str) -> bool{
    line.trim().starts_with(".")
}

pub fn extract_data(program: &str, cur_data_size: u32) -> (Vec<i32>, HashMap<String, u32>){
    let mut data = Vec::new();
    let mut data_table = HashMap::new();
    let lines: Vec<&str> = program.split("\n").collect();
    for line in lines.iter() {
        if is_data(line){
            let parts: Vec<&str> = line.split_whitespace().collect();
            match parts[0]{
                ".stringz" => { // zero terminated string
                    let string_label = &parts[1];
                    let string = &parts[2];
                    data_table.insert(string_label.to_string(), cur_data_size + data.len() as u32);
                    for val in string.chars() {
                        data.push(val as i32);
                    }
                    data.push(0);
                },
                _ => panic!("invalid data instruction")
            }
        } 
    }
    (data, data_table)
}

pub fn assemble(program: &str) -> Executable{
    assemble_and_link(vec![program])
}

pub struct Executable{
    pub code: Vec<Instruction>,
    pub data: Vec<i32>,
    pub symbol_table: HashMap<String, u32>,
    pub data_table: HashMap<String, u32>,
}

fn hashmaps_key_intersection(set1: &HashMap<String, u32>, set2: &HashMap<String, u32>) -> Vec<String>{
    let keyset1 : HashSet<String> = set1.keys().into_iter().map(|s| s.clone()).collect();
    let keyset2 : HashSet<String> = set2.keys().into_iter().map(|s| s.clone()).collect();
    keyset1.intersection(&keyset2).into_iter().map(|s| s.clone()).collect()
}

pub fn assemble_and_link(programs: Vec<&str>) -> Executable {
    let mut symbol_table = HashMap::new();
    let mut data_table = HashMap::new();
    let mut instructions = Vec::new();
    let mut data = Vec::new();
    let mut cur_rel_address = 0;
    let mut cur_data_size = 0;

    // create a symbol table for each program separately 
    // and add it to global symbol table
    // side note: we create a separate symobl table for each file instead of just concatenating all of the programs
    // in order to be able to support source-level breakpoints in the future
    for program in programs.iter(){
        let (program_symbol_table, program_size) = gen_symbol_table(*program, cur_rel_address);
        let (mut program_data, program_data_table) = extract_data(*program, cur_data_size);
        cur_rel_address += program_size;
        cur_data_size += program_data.len() as u32;
        data.append(&mut program_data);
        let symbol_intersect = hashmaps_key_intersection(&symbol_table, &program_symbol_table);
        let data_intersect = hashmaps_key_intersection(&data_table, &program_data_table);
        if symbol_intersect.len() != 0{
            panic!("duplicate symbols between programs: {:?}", symbol_intersect);
        }
        if data_intersect.len() != 0{
            panic!("duplicate data labels between programs: {:?}", data_intersect);
        }
        symbol_table.extend(program_symbol_table);
        data_table.extend(program_data_table);
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
        if let Some(instr) = maybe_parse_instruction(line, &symbol_table, &data_table, cur_rel_address) {
            instructions.push(instr);
            cur_rel_address += 1;
        } else if !is_label(line) && !is_data(line) && line.trim().len() != 0 {
            panic!("Invalid instruction: {}", line);
        }
    }
    Executable{
        code: instructions,
        data,
        symbol_table,
        data_table,
    }
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
        let exec = assemble(program);
        let isntructions = &exec.code;
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
        let exec = assemble(program);

        // println!("{:?}", symbol_table);
        assert_eq!(*exec.symbol_table.get("L1").unwrap(), 0);
        assert_eq!(*exec.symbol_table.get("L3").unwrap(), 2);
        assert_eq!(*exec.symbol_table.get("L2").unwrap(), 4);
        if let Instruction::Flow { ref op, ref offset } = exec.code[1] {
            assert_eq!(*op, FlowOp::JUMP);
            assert_eq!(*offset, 3);
        }
        if let Instruction::Flow { ref op, ref offset } = exec.code[5] {
            assert_eq!(*op, FlowOp::TJMP);
            assert_eq!(*offset, -3);
        }
    }
    #[test]
    fn test_data() {
        let program = "
        .stringz s1 hello
        .stringz s2 world
        LEA R1 s1
        ADD R1 R1 1
        LOAD R1 R1
        LEA R2 s2
        ADD R2 R2 2
        LOAD R2 R2
        ";
        let exec = assemble(program);
        assert_eq!(exec.data.len(), 12);
        assert_eq!(*exec.data_table.get("s1").unwrap(), 0);
        assert_eq!(*exec.data_table.get("s2").unwrap(), 6);
        assert_eq!(exec.data[0] , 'h' as i32);
        assert_eq!(exec.data[5] , 0);
        assert_eq!(exec.data[6] , 'w' as i32);
        assert_eq!(exec.data[11] , 0);
    }
}
