
import sys
from os import path
sys.path.append(path.dirname(path.dirname(path.dirname(path.abspath(__file__)))))
from typing import List

from pycparser import c_parser, c_ast

from cpu.instructions import BIN_ARITH_OPCODES
"""
Compiles C programs to our assembly dialect

Using pycparser to generate the AST
"""

code = []

TMP_LABEL_COUNT = 0
SCOPE_TO_DATA = {}

BIN_OP_MAP = {
    '+': 'ADD',
    '-': 'SUB',
    '*': 'MUL',
    '/': 'DIV',
    '%': 'MOD',
    '&': 'AND',
    '|': 'OR',
    '<<': 'SHL',
    '>>': 'SHR',
    '^': 'XOR',
}
assert all(arith_opcode in BIN_ARITH_OPCODES for arith_opcode in BIN_OP_MAP.values())
    
def get_ast(text):
    parser = c_parser.CParser()
    return parser.parse(text, filename='<none>')

def node_type(node):
    return node.__class__.__name__

def gen_tmp_label():
    global TMP_LABEL_COUNT
    label = f'_TMP{TMP_LABEL_COUNT}'
    TMP_LABEL_COUNT += 1
    return label

def get_binaryop_arith_opcode(op_symbol):
    assert op_symbol in BIN_OP_MAP
    return BIN_OP_MAP[op_symbol]

def right_gen(node, scope):  
    """
    invariant: in the end, evaluated value is stored in R1
    """
    ntype = node_type(node)
    if  ntype == 'Constant':
        const_val = node.value
        code.append(f'MOV R1 {const_val}')
    elif ntype == 'BinaryOp':
        right_gen(node.left, scope)
        code.append('PUSH R1')  # store result of left side on the stack
        right_gen(node.right, scope)
        code.append('POP R2')
        # now R2 = left side, R1 = right side
        if node.op in BIN_OP_MAP:
            airth_opcode = get_binaryop_arith_opcode(node.op)
            code.append(f'{airth_opcode} R1 R2 R1')
        else:
            if node.op == '==':
                code.append('TSTE R1 R2')
                code.append('MOV R1 ZR')
            if node.op == '!=':
                code.append('TSTN R1 R2')
                code.append('MOV R1 ZR')
            if node.op == '&&':
                code.append('TSTN R1 0')
                code.append('MOV R1 ZR')
                code.append('TSTN R2 0')
                code.append('AND R1 R1 ZR')
            if node.op == '||':
                code.append('TSTN R1 0')
                code.append('MOV R1 ZR')
                code.append('TSTN R2 0')
                code.append('OR R1 R1 ZR')
            if node.op == '<':
                code.append('TSTL R2 R1')
                code.append('MOV R1 ZR')
            if node.op == '<=':
                code.append('TSTG R2 R1')
                code.append('TSTN ZR 1')
                code.append('MOV R1 ZR')
            if node.op == '>':
                code.append('TSTG R2 R1')
                code.append('MOV R1 ZR')
            if node.op == '>=':
                code.append('TSTL R2 R1')
                code.append('TSTN ZR 1')
                code.append('MOV R1 ZR')

                
    elif ntype == 'UnaryOp':
        right_gen(node.expr, scope)
        if node.op == '-':
            code.append('NEG R1')
        elif node.op == '!':
            code.append('TSTE R1 0')
            code.append('MOV R1 ZR')
            # tmp_label1 = gen_tmp_label()
            # tmp_label2 = gen_tmp_label()
            # code.append('TSTE R1 0')
            # code.append(f'TJMP {tmp_label1}')
            # code.append('MOV R1 0')  # R1 != 0
            # code.append(f'JUMP {tmp_label2}')
            # code.append(f'{tmp_label1}:')
            # code.append('MOV R1 1')
            # code.append(f'{tmp_label2}:')
    elif node_type(node) == 'Assignment':
        left_gen(node.lvalue, scope)
        code.append('PUSH R1')
        right_gen(node.rvalue, scope)
        code.append('POP R2')
        if len(node.op) == 2: # e.g +=, *=
            op = node.op
            assert op[1] == '='
            opchar = op[0]
            assert opchar in BIN_OP_MAP
            # save R2 - NOTE: we could use R3 instead
            code.append('PUSH R2')
            code.append('LOAD R2 R2')  # get MEM[R2]
            code.append(f'{BIN_OP_MAP[opchar]} R1 R2 R1')
            code.append('POP R2')  # restore R2 
        code.append('STR R2 R1')
    elif node_type(node) == 'ID':
        var_name = node.name
        load_addr_of(var_name, scope)
        code.append('LOAD R1 R1')


def load_addr_of(var_name, scope):
    """
    invariant: in the end, address of variable is stored in R1
    """
    assert var_name in SCOPE_TO_DATA[scope]['vars']
    var_offset = SCOPE_TO_DATA[scope]['vars'][var_name]['offset']
    var_offset_from_bp = -(1 + len(SCOPE_TO_DATA[scope]['regs_used']) + var_offset)
    code.append(f'ADD R1 BP {var_offset_from_bp}')

def left_gen(node, scope):
    """
    invariant: in the end, evaluated address is stored in R1
    """
    if node_type(node) == 'ID':
        var_name = node.name
        load_addr_of(var_name, scope)


def code_gen(node, scope):
    if node_type(node) == 'FuncDef':
        func_name = node.decl.name
        update_vars(node.body, scope=func_name)
        # save registers
        for reg in SCOPE_TO_DATA[func_name]['regs_used']:
            code.append(f'PUSH {reg}')

        # make space on stack for local variables
        for _, var_data in SCOPE_TO_DATA[func_name]['vars'].items():
            for _ in range(var_data['size']):
                code.append(f'PUSH 0')

        code_gen(node.body, scope=func_name)

        code.append(f'_{func_name}_END:')
        # restore registers
        for reg in reversed(SCOPE_TO_DATA[func_name]['regs_used']):
            code.append(f'POP {reg}')
        code.append('RET')

    elif node_type(node) == 'Compound':
        for item in node.block_items:
            code_gen(item, scope=scope)
    elif node_type(node) == 'Return':
        right_gen(node.expr, scope)
        code.append('ADD R2 BP 2')  # TODO: assumes return value spans only a single address
        code.append('STR R2 R1')
        code.append(f'JUMP _{scope}_END')
    elif node_type(node) == 'Decl':
        if hasattr(node, 'init'):
            load_addr_of(node.name, scope)
            code.append('PUSH R1')
            right_gen(node.init, scope)
            code.append('POP R2')
            code.append('STR R2 R1')
    elif node_type(node) == 'Assignment':
        right_gen(node, scope)

def get_func_ret_type_and_size(func_node):
    return 'int', 1

def get_var_type_and_size(var_node):
    return 'int', 1 # TODO: add support for bigger types (array, structs)

def update_vars(node, scope):
    func_ret_type, func_ret_size = get_func_ret_type_and_size(node)
    SCOPE_TO_DATA[scope] = {
        'vars': {},
        'regs_used': ['R1','R2'],
        'ret_type': func_ret_type,
        'ret_size': func_ret_size
    }
    next_var_offset = 0
    for item in node.block_items:
        if node_type(item) == 'Decl':
            var_name = item.name
            var_type, var_size = get_var_type_and_size(item)
            SCOPE_TO_DATA[scope]['vars'][var_name] = {
                'type': var_type,
                'size': var_size,
                'offset': next_var_offset
            }
            next_var_offset += var_size


def compile(text : str) -> List[str]:
    global code
    code = []

    ast = get_ast(text) 

    # assuming only main function
    main_func = ast.ext[0]
    main_decl = main_func.decl
    assert main_decl.name == 'main' and main_decl.type.type.type.names[0] == 'int'
    # update_vars(main_func.body, scope='main')
    # code_gen(main_func.body, scope='main')
    code_gen(main_func, scope='')
    return code

if __name__ == "__main__":
    with open('operating_system/compiler/test_data/variables/inputs/op_assign.c') as f:
        text = f.read()
    code = '\n'.join(compile(text))
    print(code)
    from operating_system.os import run_program
    from operating_system.assembler import assemble
    run_program(assemble(code))