
import cpu.cpu as cpu
from cpu.instructions import from_str

"""
Memory layout:
0-499 os stuff
500-999 data
1000-3999 code
4000-5999 heap
6000-9999 stack
"""


"""
Stack frame:
local vars...
-----------------
reg_save (callee save)
----------------
prev_BP
ret_addr
ret_val (can span multiple addresses)
--------------
arg1
arg2
arg3

Call convention:
Calling the function:
    Caller: 
        - pushes args on the stack in reverse order
        - pushes space for return value (callee does this because distance between BP & ret val must be constant for RET instructions)
        - CALL - pushes return address (= IP + 1),
                 pushes value of current bp & updates bp=sp+1
                 jumps to function
    Callee:
        - 
        - saves all registers whose value would get destroyed
        - can allocate local vars on the stack etc.
Returning from the function:
    Callee:
        - pushes return value to the stack
        - restores values of saved registers
        - 
        - RET - SP = BP + 1
                restores BP
                jump to returna addr
"""

PROGRAM_INIT_ADDR = 1000  # TODO: change this to allow multiple programs
INIT_SP_ADDR = 9999

def load_program(instructions, init_addr):
    for instr_i, instr in enumerate(instructions):
        cpu.mem_set(init_addr + instr_i, instr )

def reset_cpu_state():
    cpu.MEM = {}
    for reg in cpu.REGS:
        cpu.REGS[reg] = 0

    cpu.MEM[0] = from_str('HALT')

def setup_stackframe():
    cpu.reg_set('SP', INIT_SP_ADDR)
    cpu.mem_set(cpu.reg_get('SP') -1,  0)  # jump to in the end HALT
    cpu.reg_set('SP', cpu.reg_get('SP') - 3)
    cpu.reg_set('BP', cpu.reg_get('SP') + 1)
    cpu.mem_set(cpu.reg_get('BP'), cpu.reg_get('BP'))  # no prev base pointer - base pointer points to itself
    cpu.mem_set(cpu.reg_get('BP') + 2, -1)  # default return value = -1


def run_program(instructions):
    load_program(instructions, PROGRAM_INIT_ADDR)
    cpu.reg_set('IP', PROGRAM_INIT_ADDR)
    # set initial stack frame
    setup_stackframe()
    cpu.start()  # run until HALT instruction
    # return the return value of the program
    return cpu.mem_get(cpu.reg_get('BP') + 1)
    

def main():
    pass

if __name__ == "__main__":
    main()