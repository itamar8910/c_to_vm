
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

if __name__ == "__main__":
    test_add()
