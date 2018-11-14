
import cpu.cpu as cpu

PROGRAM_INIT_ADDR = 400  # TODO: change this to allow multiple programs

def load_program(instructions, init_addr):
    for instr_i, instr in enumerate(instructions):
        cpu.mem_set(init_addr + instr_i, instr )

def reset_cpu_state():
    cpu.MEM = {}
    for reg in cpu.REGS:
        cpu.REGS[reg] = 0

def run_program(instructions):
    load_program(instructions, PROGRAM_INIT_ADDR)
    cpu.reg_set('IP', PROGRAM_INIT_ADDR)
    cpu.start()  # run until HALT instruction

def main():
    pass

if __name__ == "__main__":
    main()