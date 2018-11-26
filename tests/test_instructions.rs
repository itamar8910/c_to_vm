
extern crate simple_vm;

use simple_vm::operating_system::OS;
use simple_vm::cpu::instructions::Register;

#[test]
fn test_add(){
    let program = "
    MOV R1 1
    MOV R2 2
    ADD R1 R1 R2
    HALT
    ";
    let mut os = OS::new();
    let res = os.assemble_and_run(program);
    assert_eq!(res, -1);
    assert_eq!(os.cpu.regs.get(&Register::R1),3);
    assert_eq!(os.cpu.regs.get(&Register::R2),2);
}

#[test]
fn test_sub(){
    let program = "
    MOV R1 3
    MOV R2 2
    SUB R1 R1 R2
    HALT
    ";
    let mut os = OS::new();
    let res = os.assemble_and_run(program);
    assert_eq!(os.cpu.regs.get(&Register::R1),1);
}


#[test]
fn test_mul(){
    let program = "
    MOV R1 3
    MOV R2 2
    MUL R1 R1 R2
    HALT
    ";
    let mut os = OS::new();
    let res = os.assemble_and_run(program);
    assert_eq!(os.cpu.regs.get(&Register::R1),6);
}
#[test]
fn test_div(){
    let program = "
    MOV R1 5
    MOV R2 2
    DIV R1 R1 R2
    HALT
    ";
    let mut os = OS::new();
    let res = os.assemble_and_run(program);
    assert_eq!(os.cpu.regs.get(&Register::R1),2);
}
#[test]
fn test_mod(){
    let program = "
    MOV R1 5
    MOV R2 2
    MOD R1 R1 R2
    HALT
    ";
    let mut os = OS::new();
    let res = os.assemble_and_run(program);
    assert_eq!(os.cpu.regs.get(&Register::R1),1);
}
#[test]
fn test_and(){
    let program = "
    MOV R1 6
    MOV R2 3
    AND R1 R1 R2
    HALT
    ";
    let mut os = OS::new();
    let res = os.assemble_and_run(program);
    assert_eq!(os.cpu.regs.get(&Register::R1),2);
}
#[test]
fn test_or(){
    let program = "
    MOV R1 6
    MOV R2 3
    OR R1 R1 R2
    HALT
    ";
    let mut os = OS::new();
    let res = os.assemble_and_run(program);
    assert_eq!(os.cpu.regs.get(&Register::R1),7);
}
#[test]
fn test_shl(){
    let program = "
    MOV R1 6
    MOV R2 3
    SHL R1 R1 R2
    HALT
    ";
    let mut os = OS::new();
    let res = os.assemble_and_run(program);
    assert_eq!(os.cpu.regs.get(&Register::R1),48);
}

#[test]
fn test_shr(){
    let program = "
    MOV R1 6
    MOV R2 2
    SHR R1 R1 R2
    HALT
    ";
    let mut os = OS::new();
    let res = os.assemble_and_run(program);
    assert_eq!(os.cpu.regs.get(&Register::R1),1);
}

#[test]
fn test_xor(){
    let program = "
    MOV R1 6
    MOV R2 2
    XOR R1 R1 R2
    HALT
    ";
    let mut os = OS::new();
    let res = os.assemble_and_run(program);
    assert_eq!(os.cpu.regs.get(&Register::R1),4);
}
#[test]
fn test_neg(){
    let program = "
    MOV R1 4
    NEG R1
    HALT
    ";
    let mut os = OS::new();
    let res = os.assemble_and_run(program);
    assert_eq!(os.cpu.regs.get(&Register::R1),-4);
}

#[test]
fn test_add_imm(){
    let program = "
    MOV R1 2
    ADD R1 R1 3
    HALT
    ";
    let mut os = OS::new();
    let res = os.assemble_and_run(program);
    assert_eq!(os.cpu.regs.get(&Register::R1),5);
}
#[test]
fn test_str(){
    let program = "
    MOV R1 8000
    MOV R2 5
    STR R1 R2
    HALT
    ";
    let mut os = OS::new();
    let res = os.assemble_and_run(program);
    assert_eq!(os.cpu.regs.get(&Register::R2),5);
    assert_eq!(os.cpu.regs.get(&Register::R1),8000);
    assert_eq!(os.cpu.mem.get_num(8000), 5);
}



#[test]
fn test_str_imm(){
    let program = "
        MOV R1 8000
        STR R1 7
        HALT
    ";
    let mut os = OS::new();
    let res = os.assemble_and_run(program);
    assert_eq!(os.cpu.regs.get(&Register::R1),8000);
    assert_eq!(os.cpu.mem.get_num(8000), 7);
}
#[test]
fn test_load(){
    let program = "
        MOV R1 8000
        STR R1 7
        LOAD R2 R1
        HALT
    ";
    let mut os = OS::new();
    let res = os.assemble_and_run(program);
    assert_eq!(os.cpu.regs.get(&Register::R1),8000);
    assert_eq!(os.cpu.mem.get_num(8000), 7);
    assert_eq!(os.cpu.regs.get(&Register::R2),7);
}
#[test]
fn test_load_imm(){
    let program = "
        MOV R1 8000
        STR R1 7
        LOAD R2 8000
        HALT
    ";
    let mut os = OS::new();
    let res = os.assemble_and_run(program);
    assert_eq!(os.cpu.regs.get(&Register::R1),8000);
    assert_eq!(os.cpu.mem.get_num(8000), 7);
    assert_eq!(os.cpu.regs.get(&Register::R2),7);
}
#[test]
fn test_mov(){
    let program = "
        MOV R1 3
        MOV R2 R1
        HALT
    ";
    let mut os = OS::new();
    let res = os.assemble_and_run(program);
    assert_eq!(os.cpu.regs.get(&Register::R1), 3);
    assert_eq!(os.cpu.regs.get(&Register::R2), 3);
}
#[test]
fn test_mov_imm(){
    let program = "
        MOV R1 3
        HALT
    ";
    let mut os = OS::new();
    let res = os.assemble_and_run(program);
    assert_eq!(os.cpu.regs.get(&Register::R1), 3);
}

#[test]
fn test_tste_yes(){
    let program = "
        MOV R1 3
        MOV R2 3
        TSTE R1 R2 
        HALT
    ";
    let mut os = OS::new();
    let res = os.assemble_and_run(program);
    assert_eq!(os.cpu.regs.get(&Register::ZR),1);
}
#[test]
fn test_tste_no(){
    let program = "
        MOV R1 3
        MOV R2 4
        TSTE R1 R2 
        HALT
    ";
    let mut os = OS::new();
    let res = os.assemble_and_run(program);
    assert_eq!(os.cpu.regs.get(&Register::ZR),0);
}

#[test]
fn test_tste_imm_yes(){
    let program = "
        MOV R1 3
        TSTE R1 3
        HALT
    ";
    let mut os = OS::new();
    let res = os.assemble_and_run(program);
    assert_eq!(os.cpu.regs.get(&Register::ZR),1);
}


// TODO: test rest of instructions