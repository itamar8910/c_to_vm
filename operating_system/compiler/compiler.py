
from typing import List

from pycparser import c_parser, c_ast

"""
Compiles C programs to our assembly dialect

Using pycparser to generate the AST
"""

code = []

def get_ast(text):
    parser = c_parser.CParser()
    return parser.parse(text, filename='<none>')

def node_type(node):
    return node.__class__.__name__

def right_gen(node):  
    """
    invariant: at the end, evaluated value is stored in R1
    """
    if node_type(node) == 'Constant':
        const_val = node.value
        code.append(f'MOV R1 {const_val}')

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
    with open('operating_system/compiler/data/simple.c') as f:
        text = f.read()
    print(compile(text))