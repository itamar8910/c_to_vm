
"""
arg1 must be a register
arg2 must be a register
arg3 can be either a register or an immediate value
"""
ARITH_OPCODES = {
    'ADD' : lambda x, y: x + y,
    'SUB': lambda x, y: x - y,
    'MUL': lambda x, y: x * y,
    'DIV': lambda x, y: x // y,
    'MOD': lambda x,y : x % y,
    'AND': lambda x, y: x & y,
    'OR': lambda x, y : x | y,
    'SHL': lambda x, y: x << y,
    'SHR': lambda x, y: x >> y
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

    # PUSH Rx: MEM[SP] = Rx, SP-=1
    'PUSH': None,
    # POP Rx: Rx = MEM[SP], SP+=1
    'POP': None,
}

TEST_OPCODES = {
    'TSTE': lambda x, y : int(x == y),
    'TSTG': lambda x, y: int(x > y),
    'TSTL': lambda x, y: int(x < y),
}
# instructions that affect IR (excluding RET)
FLOW_OPCODES = {
    'JUMP': lambda val : True,
    'TJMP': lambda val : val ,
    'FJMP': lambda val : not val,
    'CALL': lambda val : True
}

SPECIAL_OPCODES = {
    'HALT' : None,
    'RET' : None
}


def to_str(instruction):
    opcode = instruction['op']
    if opcode in ARITH_OPCODES:
        dst = instruction['dst']
        arg1 = instruction['arg1']
        arg2 = instruction['arg2']
        return f'{opcode} {dst} {arg1} {arg2}' 
    if opcode in DATA_OPCODES:
        if opcode == 'PUSH':
            return f"{opcode} {instruction['src']}"
        if opcode == 'POP':
            return f"{opcode} {instruction['dst']}"
        src = instruction['src']
        dst = instruction['dst']
        return f'{opcode} {dst} {src}'
    if opcode in TEST_OPCODES:
        arg1 = instruction['arg1']
        arg2 = instruction['arg2']
        return f'{opcode} {arg1} {arg2}'
    if opcode in FLOW_OPCODES or opcode in SPECIAL_OPCODES:
        return f'{opcode}'

def from_str(s):
    def maybe_cast_arg(arg):
        "this will try to cast arg to int, otherwise assumes its a register"
        try:
            return int(arg)
        except ValueError:
            return arg

    args = s.split()
    opcode = args[0]
    if opcode in ARITH_OPCODES:
        dst = args[1]
        arg1 = args[2] 
        arg2 = args[3] 
        return {'op': opcode, 'dst': dst, 'arg1': arg1, 'arg2': maybe_cast_arg(arg2)}
    if opcode in DATA_OPCODES:
        if opcode == 'PUSH':
            return {'op': opcode, 'src': maybe_cast_arg(args[1])}
        if opcode == 'POP':
            return {'op': opcode, 'dst': args[1]}
        dst = args[1]
        src = args[2] 
        return {'op': opcode, 'dst': dst, 'src': maybe_cast_arg(src)}
    if opcode in TEST_OPCODES:
        arg1 = args[1] 
        arg2 = args[2] 
        return {'op': opcode, 'arg1': arg1, 'arg2': maybe_cast_arg(arg2)}
    if opcode in FLOW_OPCODES:
        return {'op': opcode, 'offset': args[1]}
    if opcode in SPECIAL_OPCODES:
        return {'op': opcode}
    raise Exception('INVALID OPCODE:{}'.format(opcode))


