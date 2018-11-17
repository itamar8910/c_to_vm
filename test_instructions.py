
from cpu.cpu import reg_get, mem_get
from cpu.instructions import to_str
from operating_system.os import run_program 
from operating_system.assembler import assemble

def test_add():
    program = """
    MOV R1 1
    MOV R2 2
    ADD R1 R1 R2
    HALT
    """
    run_program(assemble(program))
    assert reg_get('R1') == 3

def test_sub():
    program = """
    MOV R1 3
    MOV R2 2
    SUB R1 R1 R2
    HALT
    """
    run_program(assemble(program))
    assert reg_get('R1') == 1


def test_mul():
    program = """
    MOV R1 3
    MOV R2 2
    MUL R1 R1 R2
    HALT
    """
    run_program(assemble(program))
    assert reg_get('R1') == 6

def test_div():
    program = """
    MOV R1 5
    MOV R2 2
    DIV R1 R1 R2
    HALT
    """
    run_program(assemble(program))
    assert reg_get('R1') == 2

def test_mod():
    program = """
    MOV R1 5
    MOV R2 2
    MOD R1 R1 R2
    HALT
    """
    run_program(assemble(program))
    assert reg_get('R1') == 1

def test_and():
    program = """
    MOV R1 6
    MOV R2 3
    AND R1 R1 R2
    HALT
    """
    run_program(assemble(program))
    assert reg_get('R1') == 2

def test_or():
    program = """
    MOV R1 6
    MOV R2 3
    OR R1 R1 R2
    HALT
    """
    run_program(assemble(program))
    assert reg_get('R1') == 7

def test_shl():
    program = """
    MOV R1 6
    MOV R2 3
    SHL R1 R1 R2
    HALT
    """
    run_program(assemble(program))
    assert reg_get('R1') == 48

def test_shr():
    program = """
    MOV R1 6
    MOV R2 2
    SHR R1 R1 R2
    HALT
    """
    run_program(assemble(program))
    assert reg_get('R1') == 1

def test_neg():
    program = """
    MOV R1 4
    NEG R1
    HALT
    """
    run_program(assemble(program))
    assert reg_get('R1') == -4

def test_add_imm():
    program = """
    MOV R1 2
    ADD R1 R1 3
    HALT
    """
    run_program(assemble(program))
    assert reg_get('R1') == 5

def test_str():
    program = """
        MOV R1 8000
        MOV R2 5
        STR R1 R2
        HALT
    """
    run_program(assemble(program))
    assert reg_get('R2') == 5
    assert reg_get('R1') == 8000
    assert mem_get(reg_get('R1')) == reg_get('R2')

def test_str_imm():
    program = """
        MOV R1 8000
        STR R1 7
        HALT
    """
    run_program(assemble(program))
    assert reg_get('R1') == 8000
    assert mem_get(reg_get('R1')) == 7

def test_load():
    program = """
        MOV R1 8000
        STR R1 7
        LOAD R2 R1
        HALT
    """
    run_program(assemble(program))
    assert reg_get('R1') == 8000
    assert mem_get(reg_get('R1')) == 7
    assert reg_get('R2') == 7

def test_load_imm():
    program = """
        MOV R1 8000
        STR R1 7
        LOAD R2 8000
        HALT
    """
    run_program(assemble(program))
    assert reg_get('R1') == 8000
    assert mem_get(reg_get('R1')) == 7
    assert reg_get('R2') == 7

def test_mov():
    program = """
        MOV R1 3
        MOV R2 R1
        HALT
    """
    run_program(assemble(program))
    assert reg_get('R1') == 3
    assert reg_get('R2') == 3

def test_mov_imm():
    program = """
        MOV R1 3
        HALT
    """
    run_program(assemble(program))
    assert reg_get('R1') == 3

def test_tste_yes():
    program = """
        MOV R1 3
        MOV R2 3
        TSTE R1 R2 
        HALT
    """
    run_program(assemble(program))
    assert reg_get('ZR') == 1

def test_tste_no():
    program = """
        MOV R1 3
        MOV R2 4
        TSTE R1 R2 
        HALT
    """
    run_program(assemble(program))
    assert reg_get('ZR') == 0
 

def test_tste_imm_yes():
    program = """
        MOV R1 3
        TSTE R1 3
        HALT
    """
    run_program(assemble(program))
    assert reg_get('ZR') == 1

def test_tstg_yes():
    program = """
        MOV R1 4
        MOV R2 3
        TSTG R1 R2 
        HALT
    """
    run_program(assemble(program))
    assert reg_get('ZR') == 1

def test_tstg_no():
    program = """
        MOV R1 3
        MOV R2 4
        TSTG R1 R2 
        HALT
    """
    run_program(assemble(program))
    assert reg_get('ZR') == 0

def test_tstl_yes():
    program = """
        MOV R1 2
        MOV R2 3
        TSTL R1 R2 
        HALT
    """
    run_program(assemble(program))
    assert reg_get('ZR') == 1

def test_tstl_no():
    program = """
        MOV R1 3
        MOV R2 2
        TSTL R1 R2 
        HALT
    """
    run_program(assemble(program))
    assert reg_get('ZR') == 0

def test_rel_address():
    program = """
        MOV R1 3
        JUMP SKIP
        MOV R1 4
        SKIP:
        HALT
    """
    instructions, symbol_table = assemble(program, ret_symbol_table=True)
    assert symbol_table['SKIP'] == 3
    assert instructions[1]['offset'] == 2

def test_jump():
    program = """
        MOV R1 3
        JUMP SKIP
        MOV R1 4
        SKIP:
        HALT
    """
    run_program(assemble(program))
    assert reg_get('R1') == 3

def test_jump2():
    program = """
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
    """
    run_program(assemble(program))
    assert reg_get('R1') == 4

def test_tjump_pos():
    program = """
        MOV R1 3
        TSTE R1 3
        TJMP SKIP
        MOV R1 4
        SKIP:
        HALT
    """
    run_program(assemble(program))
    assert reg_get('R1') == 3

def test_tjump_neg():
    program = """
        MOV R1 3
        TSTE R1 2
        TJMP SKIP
        MOV R1 4
        SKIP:
        HALT
    """
    run_program(assemble(program))
    assert reg_get('R1') == 4

def test_fjump_pos():
    program = """
        MOV R1 3
        TSTE R1 2
        FJMP SKIP
        MOV R1 4
        SKIP:
        HALT
    """
    run_program(assemble(program))
    assert reg_get('R1') == 3

def test_fjump_neg():
    program = """
        MOV R1 3
        TSTE R1 3
        FJMP SKIP
        MOV R1 4
        SKIP:
        HALT
    """
    run_program(assemble(program))
    assert reg_get('R1') == 4

def test_push_imm():
    program = """
        PUSH 1 
        PUSH 2
        POP R1
        HALT
    """
    run_program(assemble(program))
    assert reg_get('R1') == 2

def test_push():
    program = """
        MOV R2 3
        PUSH 1 
        PUSH R2
        POP R1
        HALT
    """
    run_program(assemble(program))
    assert reg_get('R1') == 3

def test_pop():
    program = """
        MOV R1 4
        PUSH 1 
        PUSH R1
        POP R2
        POP R1 
        HALT
    """
    run_program(assemble(program))
    assert reg_get('R2') == 4
    assert reg_get('R1') == 1

def test_call_ret_simple():
    program = """
        JUMP MAIN
        FOO:
        MOV R2 2
        RET
        MAIN:
        MOV R1  1
        CALL FOO
        MOV R3 3
        HALT
    """
    run_program(assemble(program))
    assert reg_get('R1') == 1
    assert reg_get('R2') == 2
    assert reg_get('R3') == 3

def test_call_ret_with_args_and_retval():
    program = """
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
        PUSH 1
        PUSH 2
        PUSH 0
        CALL ADD
        POP R1
        HALT
    """
    run_program(assemble(program))
    assert reg_get('R1') == 3

def test_call_multiple():
    program = """
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
    """
    run_program(assemble(program))
    assert reg_get('R1') == 5

def test_recursion_fiboncci():
    program = """
    JUMP MAIN
    MAIN:
    PUSH 6 
    PUSH 0
    CALL FIBBO
    POP R1
    HALT
    FIBBO:
    PUSH R1
    PUSH R2
    PUSH R5
    ADD R5 BP 3
    LOAD R5 R5
    TSTG R5 1
    TJMP RECURSE
    ADD R1 BP 2
    STR R1 R5
    JUMP FIBO_RET
    RECURSE:
    ADD R5 R5 -1
    PUSH R5 
    PUSH 0
    CALL FIBBO
    POP R1
    POP R5
    ADD R5 R5 -1
    PUSH R5
    PUSH 0
    CALL FIBBO
    POP R2
    POP R5
    ADD R1 R1 R2
    ADD R2 BP 2
    STR R2 R1
    FIBO_RET:
    POP R5
    POP R2
    POP R1
    RET
    """
    run_program(assemble(program))
    assert reg_get('R1') == 8

def test_program_ret_val():
    program = """
    MAIN:
    ADD R1 BP 2
    STR R1 3
    RET
    """
    ret_val = run_program(assemble(program))
    assert ret_val == 3

test_program_ret_val()