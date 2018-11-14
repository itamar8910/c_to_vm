
from cpu.cpu import reg_get, mem_get
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


# TODO: test tjump, fjump
if __name__ == "__main__":
    test_str_imm()