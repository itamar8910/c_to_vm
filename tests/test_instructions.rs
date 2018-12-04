extern crate simple_vm;

use simple_vm::cpu::instructions::Register;
use simple_vm::operating_system::OS;

#[test]
fn test_add() {
    let program = "
    MOV R1 1
    MOV R2 2
    ADD R1 R1 R2
    HALT
    ";
    let mut os = OS::new();
    let res = os.assemble_and_run(program);
    assert_eq!(res, -1);
    assert_eq!(os.cpu.regs.get(&Register::R1), 3);
    assert_eq!(os.cpu.regs.get(&Register::R2), 2);
}

#[test]
fn test_sub() {
    let program = "
    MOV R1 3
    MOV R2 2
    SUB R1 R1 R2
    HALT
    ";
    let mut os = OS::new();
    let res = os.assemble_and_run(program);
    assert_eq!(os.cpu.regs.get(&Register::R1), 1);
}

#[test]
fn test_mul() {
    let program = "
    MOV R1 3
    MOV R2 2
    MUL R1 R1 R2
    HALT
    ";
    let mut os = OS::new();
    let res = os.assemble_and_run(program);
    assert_eq!(os.cpu.regs.get(&Register::R1), 6);
}
#[test]
fn test_div() {
    let program = "
    MOV R1 5
    MOV R2 2
    DIV R1 R1 R2
    HALT
    ";
    let mut os = OS::new();
    let res = os.assemble_and_run(program);
    assert_eq!(os.cpu.regs.get(&Register::R1), 2);
}
#[test]
fn test_mod() {
    let program = "
    MOV R1 5
    MOV R2 2
    MOD R1 R1 R2
    HALT
    ";
    let mut os = OS::new();
    let res = os.assemble_and_run(program);
    assert_eq!(os.cpu.regs.get(&Register::R1), 1);
}
#[test]
fn test_and() {
    let program = "
    MOV R1 6
    MOV R2 3
    AND R1 R1 R2
    HALT
    ";
    let mut os = OS::new();
    let res = os.assemble_and_run(program);
    assert_eq!(os.cpu.regs.get(&Register::R1), 2);
}
#[test]
fn test_or() {
    let program = "
    MOV R1 6
    MOV R2 3
    OR R1 R1 R2
    HALT
    ";
    let mut os = OS::new();
    let res = os.assemble_and_run(program);
    assert_eq!(os.cpu.regs.get(&Register::R1), 7);
}
#[test]
fn test_shl() {
    let program = "
    MOV R1 6
    MOV R2 3
    SHL R1 R1 R2
    HALT
    ";
    let mut os = OS::new();
    let res = os.assemble_and_run(program);
    assert_eq!(os.cpu.regs.get(&Register::R1), 48);
}

#[test]
fn test_shr() {
    let program = "
    MOV R1 6
    MOV R2 2
    SHR R1 R1 R2
    HALT
    ";
    let mut os = OS::new();
    let res = os.assemble_and_run(program);
    assert_eq!(os.cpu.regs.get(&Register::R1), 1);
}

#[test]
fn test_xor() {
    let program = "
    MOV R1 6
    MOV R2 2
    XOR R1 R1 R2
    HALT
    ";
    let mut os = OS::new();
    let res = os.assemble_and_run(program);
    assert_eq!(os.cpu.regs.get(&Register::R1), 4);
}
#[test]
fn test_neg() {
    let program = "
    MOV R1 4
    NEG R1
    HALT
    ";
    let mut os = OS::new();
    let res = os.assemble_and_run(program);
    assert_eq!(os.cpu.regs.get(&Register::R1), -4);
}

#[test]
fn test_add_imm() {
    let program = "
    MOV R1 2
    ADD R1 R1 3
    HALT
    ";
    let mut os = OS::new();
    let res = os.assemble_and_run(program);
    assert_eq!(os.cpu.regs.get(&Register::R1), 5);
}
#[test]
fn test_str() {
    let program = "
    MOV R1 8000
    MOV R2 5
    STR R1 R2
    HALT
    ";
    let mut os = OS::new();
    let res = os.assemble_and_run(program);
    assert_eq!(os.cpu.regs.get(&Register::R2), 5);
    assert_eq!(os.cpu.regs.get(&Register::R1), 8000);
    assert_eq!(os.cpu.mem.get_num(8000), 5);
}

#[test]
fn test_str_imm() {
    let program = "
        MOV R1 8000
        STR R1 7
        HALT
    ";
    let mut os = OS::new();
    let res = os.assemble_and_run(program);
    assert_eq!(os.cpu.regs.get(&Register::R1), 8000);
    assert_eq!(os.cpu.mem.get_num(8000), 7);
}
#[test]
fn test_load() {
    let program = "
        MOV R1 8000
        STR R1 7
        LOAD R2 R1
        HALT
    ";
    let mut os = OS::new();
    let res = os.assemble_and_run(program);
    assert_eq!(os.cpu.regs.get(&Register::R1), 8000);
    assert_eq!(os.cpu.mem.get_num(8000), 7);
    assert_eq!(os.cpu.regs.get(&Register::R2), 7);
}
#[test]
fn test_load_imm() {
    let program = "
        MOV R1 8000
        STR R1 7
        LOAD R2 8000
        HALT
    ";
    let mut os = OS::new();
    let res = os.assemble_and_run(program);
    assert_eq!(os.cpu.regs.get(&Register::R1), 8000);
    assert_eq!(os.cpu.mem.get_num(8000), 7);
    assert_eq!(os.cpu.regs.get(&Register::R2), 7);
}
#[test]
fn test_mov() {
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
fn test_mov_imm() {
    let program = "
        MOV R1 3
        HALT
    ";
    let mut os = OS::new();
    let res = os.assemble_and_run(program);
    assert_eq!(os.cpu.regs.get(&Register::R1), 3);
}

#[test]
fn test_tste_yes() {
    let program = "
        MOV R1 3
        MOV R2 3
        TSTE R1 R2 
        HALT
    ";
    let mut os = OS::new();
    let res = os.assemble_and_run(program);
    assert_eq!(os.cpu.regs.get(&Register::ZR), 1);
}
#[test]
fn test_tste_no() {
    let program = "
        MOV R1 3
        MOV R2 4
        TSTE R1 R2 
        HALT
    ";
    let mut os = OS::new();
    let res = os.assemble_and_run(program);
    assert_eq!(os.cpu.regs.get(&Register::ZR), 0);
}

#[test]
fn test_tste_imm_yes() {
    let program = "
        MOV R1 3
        TSTE R1 3
        HALT
    ";
    let mut os = OS::new();
    let res = os.assemble_and_run(program);
    assert_eq!(os.cpu.regs.get(&Register::ZR), 1);
}

// tests bellow were converted from py test with regex

#[test]
fn test_tstg_yes() {
    let program = "
        MOV R1 4
        MOV R2 3
        TSTG R1 R2 
        HALT
    ";
    let mut os = OS::new();
    let res = os.assemble_and_run(program);
    assert_eq!(os.cpu.regs.get(&Register::ZR), 1);
}

#[test]
fn test_tstg_no() {
    let program = "
        MOV R1 3
        MOV R2 4
        TSTG R1 R2 
        HALT
    ";
    let mut os = OS::new();
    let res = os.assemble_and_run(program);
    assert_eq!(os.cpu.regs.get(&Register::ZR), 0);
}

#[test]
fn test_tstl_yes() {
    let program = "
        MOV R1 2
        MOV R2 3
        TSTL R1 R2 
        HALT
    ";
    let mut os = OS::new();
    let res = os.assemble_and_run(program);
    assert_eq!(os.cpu.regs.get(&Register::ZR), 1);
}

#[test]
fn test_tstl_no() {
    let program = "
        MOV R1 3
        MOV R2 2
        TSTL R1 R2 
        HALT
    ";
    let mut os = OS::new();
    let res = os.assemble_and_run(program);
    assert_eq!(os.cpu.regs.get(&Register::ZR), 0);
}

// #[test]
// fn test_rel_address(){
//     let program = "
//         MOV R1 3
//         JUMP SKIP
//         MOV R1 4
//         SKIP:
//         HALT
//     ";
// 	let mut os = OS::new();
// 	let res = os.assemble_and_run(program);
// }

#[test]
fn test_jump() {
    let program = "
        MOV R1 3
        JUMP SKIP
        MOV R1 4
        SKIP:
        HALT
    ";
    let mut os = OS::new();
    let res = os.assemble_and_run(program);
    assert_eq!(os.cpu.regs.get(&Register::R1), 3);
}
#[test]
fn test_jump2() {
    let program = "
        MOV R1 3
        JUMP SKIP
        L2:
        MOV R1 4
        JUMP END
        SKIP:
        JUMP L2
        MOV R1 5
        END:
        HALT
    ";
    let mut os = OS::new();
    let res = os.assemble_and_run(program);
    assert_eq!(os.cpu.regs.get(&Register::R1), 4);
}

#[test]
fn test_tjump_pos() {
    let program = "
        MOV R1 3
        TSTE R1 3
        TJMP SKIP
        MOV R1 4
        SKIP:
        HALT
    ";
    let mut os = OS::new();
    let res = os.assemble_and_run(program);
    assert_eq!(os.cpu.regs.get(&Register::R1), 3);
}

#[test]
fn test_tjump_neg() {
    let program = "
        MOV R1 3
        TSTE R1 2
        TJMP SKIP
        MOV R1 4
        SKIP:
        HALT
    ";
    let mut os = OS::new();
    let res = os.assemble_and_run(program);
    assert_eq!(os.cpu.regs.get(&Register::R1), 4);
}

#[test]
fn test_fjump_pos() {
    let program = "
        MOV R1 3
        TSTE R1 2
        FJMP SKIP
        MOV R1 4
        SKIP:
        HALT
    ";
    let mut os = OS::new();
    let res = os.assemble_and_run(program);
    assert_eq!(os.cpu.regs.get(&Register::R1), 3);
}

#[test]
fn test_fjump_neg() {
    let program = "
        MOV R1 3
        TSTE R1 3
        FJMP SKIP
        MOV R1 4
        SKIP:
        HALT
    ";
    let mut os = OS::new();
    let res = os.assemble_and_run(program);
    assert_eq!(os.cpu.regs.get(&Register::R1), 4);
}

#[test]
fn test_push_imm() {
    let program = "
        MOV R1 1
        PUSH R1 
        MOV R2 2
        PUSH R2
        POP R1
        HALT
    ";
    let mut os = OS::new();
    let res = os.assemble_and_run(program);
    assert_eq!(os.cpu.regs.get(&Register::R1), 2);
}

#[test]
fn test_push() {
    let program = "
        MOV R1 1
        MOV R2 3
        PUSH R1 
        PUSH R2
        POP R1
        HALT
    ";
    let mut os = OS::new();
    let res = os.assemble_and_run(program);
    assert_eq!(os.cpu.regs.get(&Register::R1), 3);
}

#[test]
fn test_pop() {
    let program = "
        MOV R2 1
        MOV R1 4
        PUSH R2 
        PUSH R1
        POP R2
        POP R1 
        HALT
    ";
    let mut os = OS::new();
    let res = os.assemble_and_run(program);
    assert_eq!(os.cpu.regs.get(&Register::R2), 4);
    assert_eq!(os.cpu.regs.get(&Register::R1), 1);
}

#[test]
fn test_call_ret_simple() {
    let program = "
        JUMP MAIN
        FOO:
        MOV R2 2
        RET
        MAIN:
        MOV R1  1
        CALL FOO
        MOV R3 3
        HALT
    ";
    let mut os = OS::new();
    let res = os.assemble_and_run(program);
    assert_eq!(os.cpu.regs.get(&Register::R1), 1);
    assert_eq!(os.cpu.regs.get(&Register::R2), 2);
    assert_eq!(os.cpu.regs.get(&Register::R3), 3);
}

#[test]
fn test_call_ret_with_args_and_retval() {
    let program = "
        JUMP MAIN
        ADD:
        ADD R1 BP 3
        LOAD R1 R1
        ADD R2 BP 4
        LOAD R2 R2
        ADD R1 R1 R2
        ADD R2 BP 2
        STR R2 R1
        RET
        MAIN:
        MOV R3 1
        PUSH R3
        MOV R3 2
        PUSH R3
        MOV R3 0
        PUSH R3
        CALL ADD
        POP R1
        HALT
    ";
    let mut os = OS::new();
    let res = os.assemble_and_run(program);
    assert_eq!(os.cpu.regs.get(&Register::R1), 3);
}

#[test]
fn test_call_multiple() {
    let program = "
    JUMP MAIN
    MAIN:
    CALL FOO1
    MOV R1 5
    HALT
    FOO2:
    CALL FOO3
    RET
    FOO1:
    CALL FOO2
    RET
    FOO3:
    RET
    ";
    let mut os = OS::new();
    let res = os.assemble_and_run(program);
    assert_eq!(os.cpu.regs.get(&Register::R1), 5);
}

#[test]
fn test_recursion_fiboncci() {
    let program = "
    JUMP MAIN
    MAIN:
    MOV R3 6
    PUSH R3 
    MOV R3 0
    PUSH R3
    CALL FIBBO
    POP R1
    HALT
    FIBBO:
    PUSH R1
    PUSH R2
    PUSH R4
    ADD R4 BP 3
    LOAD R4 R4
    TSTG R4 1
    TJMP RECURSE
    ADD R1 BP 2
    STR R1 R4
    JUMP FIBO_RET
    RECURSE:
    ADD R4 R4 -1
    PUSH R4 
    MOV R3 0
    PUSH R3
    CALL FIBBO
    POP R1
    POP R4
    ADD R4 R4 -1
    PUSH R4
    MOV R3 0
    PUSH R3
    CALL FIBBO
    POP R2
    POP R4
    ADD R1 R1 R2
    ADD R2 BP 2
    STR R2 R1
    FIBO_RET:
    POP R4
    POP R2
    POP R1
    RET
    ";
    let mut os = OS::new();
    let res = os.assemble_and_run(program);
    assert_eq!(os.cpu.regs.get(&Register::R1), 8);
}

#[test]
fn test_program_ret_val() {
    let program = "
    MAIN:
    ADD R1 BP 2
    STR R1 3
    RET
    ";
    let mut os = OS::new();
    let res = os.assemble_and_run(program);
    assert_eq!(res, 3);
}
