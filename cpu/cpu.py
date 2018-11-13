
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
       'ZR' : 0, # contains the boolean result of the TEST instruction
    }
} 


"""
arg1 must be a register
arg2 must be a register
arg3 can be either a register or an immediate value
"""
ARITH_OPCODES = {
    'ADD' : lambda x, y: x + y,
    'SUB': lambda x, y: x - y,
    'MUL': lambda x, y: x * y,
    'DIV': lambda x, y: x / y,
    'MOD': lambda x,y : x % y,
    'AND': lambda x, y: x & y,
    'OR': lambda x, y : x | y,
    'SHR': lambda x, y: x << y,
    'SHL': lambda x, y: x >> y
    }

DATA_OPCODES = {
    # store in memory
    # arg1 must be a register, arg2 can be either register or immediate
    # MEM[arg1] = arg2
    'STR': None,  

    # load from memory
    # arg1 must be a register 
    # args2 can be either register or immediate
    # REG['arg1'] = MEM['arg2']
    'LOAD': None, # load from memory

    # move value to register  
    # arg1 must be a register
    # arg2 can be either a register or an immediate
    # REG[arg1] = arg2
    'MOV': None,
}

TEST_OPCODES = {
    'TSTE': lambda x, y : x == y,
    'TSTG': lambda x, y: x > y,
    'TSTL': lambda x, y: x < y,
}
# instructions that affect IR
FLOW_OPCODES = {
    'JUMP': lambda : True,
    'TJMP': lambda : REGS['ZR'] ,
    'FJMP': lambda : not REGS['ZR']
}

def valid_address(address):
    return address >= 0 and address < MEM_SIZE

def mem_set(val, address):
    assert valid_address(address)
    assert type(val) in [dict, int, float]
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
    dst = opcode['dst']
    arg1 = opcode['arg1']
    arg2 = opcode['arg2']
    assert dst in REGS
    assert arg1 in REGS
    arg1_val = reg_get(arg1)
    # arg2 is either a register or an immediate
    arg2_val = reg_get(arg2) if arg2 in REGS else arg2
    res_val = ARITH_OPCODES[opcode](arg1_val, arg2_val)
    reg_set(dst, res_val)

def execute_data(instruction):
    opcode = instruction['op']
    dst = instruction['dst']
    src = instruction['src']
    assert dst in REGS
    src_val = reg_get(src) if src in REGS else src
    if opcode == 'STR':
        dst_val = reg_get(dst)
        mem_set(dst_val, src_val)
    elif opcode == 'LOAD':
        reg_set(dst, mem_get(src_val))
    elif opcode == 'MOV':
        reg_set(dst, src_val)


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
    arg = instruction['arg']
    if FLOW_OPCODES[opcode]:
        REGS['IP'] = arg - 1  # -1 because IP is incrementd at end of the cpu cycle in any case

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
    raise Exception('Invalid instruction:{}'.format(instruction)) 

def start():
    while True:
       cur_instruction = fetch()
       execute(cur_instruction)
       REGS['IP'] = REGS['IP'] + 1



if __name__ == "__main__":
    start()





