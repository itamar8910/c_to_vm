
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

BIN_OP_MAP = {
    '+': 'ADD',
    '-': 'SUB',
    '*': 'MUL',
    '/': 'DIV',
    '%': 'MOD',
    '&': 'AND',
    '|': 'OR',
    '<<': 'SHL',
    '>>': 'SHR'
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

def right_gen(node):  
    """
    invariant: in the end, evaluated value is stored in R1
    """
    ntype = node_type(node)
    if  ntype == 'Constant':
        const_val = node.value
        code.append(f'MOV R1 {const_val}')
    elif ntype == 'BinaryOp':
        right_gen(node.left)
        code.append('PUSH R1')  # store result of left side on the stack
        right_gen(node.right)
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
        right_gen(node.expr)
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



def code_gen(node):
    if node_type(node) == 'Compound':
        for item in node.block_items:
            code_gen(item)
    elif node_type(node) == 'Return':
        right_gen(node.expr)
        code.append('ADD R2 BP 2')  # TODO: assumes return value spans only a single address
        code.append('STR R2 R1')
        code.append('RET')


def compile(text : str) -> List[str]:
    global code
    code = []

    ast = get_ast(text) 

    # assuming only main function
    main_func = ast.ext[0]
    main_decl = main_func.decl
    assert main_decl.name == 'main' and main_decl.type.type.type.names[0] == 'int'
    code_gen(main_func.body)
    return code

if __name__ == "__main__":
    with open('operating_system/compiler/test_data/bool_expressions/inputs/ge_false.c') as f:
        text = f.read()
    code = '\n'.join(compile(text))
    print(code)
    from operating_system.os import run_program
    from operating_system.assembler import assemble
    run_program(assemble(code))