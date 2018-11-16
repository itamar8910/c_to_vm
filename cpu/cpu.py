
from cpu.instructions import ARITH_OPCODES, DATA_OPCODES, TEST_OPCODES, FLOW_OPCODES, SPECIAL_OPCODES
from cpu.instructions import to_str

running = True
NUM_REGISTERS = 8  # number of general purpose registers
MEM_SIZE = 10000



"""
our memory is magical
it can store data of any size in each of its addresses
each address in memory stores either a dict(=instruction) or number
so we store instructions as dict!
"""
MEM = {}
REGS = {
    **{
        'R{}'.format(str(i)): 0 for i in range(1, NUM_REGISTERS + 1)
    },
    **{
       'IP': 0, # instruction pointer
       'SP': 0, # stack pointer
       'BP': 0, # base pointer
       'ZR' : 0, # contains the boolean(1 or 0) result of the last TEST instruction
    }
} 



def valid_address(address):
    return address >= 0 and address < MEM_SIZE

def mem_set(address, val):
    assert valid_address(address)
    assert type(val) in [dict, int]
    MEM[address] = val

def mem_get(address):
    assert valid_address(address)
    assert address in MEM  # boy will this help debugging
    return MEM[address]

def reg_set(reg, val):
    REGS[reg] = val

def reg_get(reg):
    return REGS[reg]
    

def fetch():
    return MEM[REGS['IP']] 

def execute_arith(instruction):
    opcode = instruction['op']
    dst = instruction['dst']
    arg1 = instruction['arg1']
    arg2 = instruction['arg2']
    assert dst in REGS
    assert arg1 in REGS
    arg1_val = reg_get(arg1)
    # arg2 is either a register or an immediate
    arg2_val = reg_get(arg2) if arg2 in REGS else arg2
    res_val = ARITH_OPCODES[opcode](arg1_val, arg2_val)
    reg_set(dst, res_val)

def execute_data(instruction):
    opcode = instruction['op']
    dst = instruction.get('dst', None)
    src = instruction.get('src', None)
    if dst:
        assert dst in REGS
    if src is not None:
        src_val = reg_get(src) if src in REGS else src
    if opcode == 'STR':
        dst_val = reg_get(dst)
        mem_set(dst_val, src_val)
    elif opcode == 'LOAD':
        reg_set(dst, mem_get(src_val))
    elif opcode == 'MOV':
        reg_set(dst, src_val)
    elif opcode == 'PUSH':
        MEM[REGS['SP']] = src_val
        REGS['SP'] -= 1
    elif opcode == 'POP':
        reg_set(dst, MEM[REGS['SP']+1])
        REGS['SP'] += 1
    


def execute_test(instruction):
    opcode = instruction['op']
    arg1 = instruction['arg1']
    arg2 = instruction['arg2']
    assert arg1 in REGS
    arg1_val = reg_get(arg1)
    arg2_val = reg_get(arg2) if arg2 in REGS else arg2
    res = TEST_OPCODES[opcode](arg1_val, arg2_val)
    reg_set('ZR', res)


def execute_flow(instruction):
    opcode = instruction['op']
    offset = instruction['offset']
    if FLOW_OPCODES[opcode](REGS['ZR']):  # call, jump are always taken
        if opcode == 'CALL':
            MEM[REGS['SP']] = REGS['IP'] + 1  # push ret addr
            MEM[REGS['SP'] - 1] = REGS['BP']  # save caller BP
            REGS['BP'] = REGS['SP'] - 1
            REGS['SP'] -= 2
        REGS['IP'] += offset - 1  # -1 because IP will be incrementd at end of the cpu cycle in any case

def execute_special(instruction):
    opcode = instruction['op']
    if opcode == 'HALT':
        global running
        running = False
    if opcode == 'RET':
        REGS['SP'] = REGS['BP'] + 1
        ret_addr = MEM[REGS['BP'] + 1]
        REGS['BP'] = MEM[REGS['BP']]
        REGS['IP'] = ret_addr - 1  # because IP will be increment at end of cycle






def execute(instruction):
    opcode = instruction['op']
    if opcode in ARITH_OPCODES:
        execute_arith(instruction)
    elif opcode in DATA_OPCODES:
        execute_data(instruction)
    elif opcode in TEST_OPCODES:
        execute_test(instruction)
    elif opcode in FLOW_OPCODES:
        execute_flow(instruction)
    elif opcode in SPECIAL_OPCODES:
        execute_special(instruction)
    else:
        raise Exception('Invalid instruction:{}'.format(instruction)) 

def start():
    global running
    running = True
    while running:
        cur_instruction = fetch()
        print(to_str(cur_instruction))
        execute(cur_instruction)
        REGS['IP'] = REGS['IP'] + 1



# if __name__ == "__main__":
#     start()





