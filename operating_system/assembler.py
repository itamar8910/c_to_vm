from cpu.instructions import from_str, FLOW_OPCODES

def is_label(line):
    return ':' in line

def get_label_val(line):
    return line.strip().replace(':', '')

def is_instruction(line):
    return not is_label(line) and line.strip()

def assemble(program, ret_symbol_table = False):
    """
    assebles given program
    returns instructions
    """
    symbol_table = {}
    instructions = []
    cur_rel_address = 0
    # first pass, create symbol table
    # TODO: impl. data section & add it to symbol table
    for line in program.split('\n'):
        if is_label(line):
            label = get_label_val(line)
            symbol_table[label] = cur_rel_address
        elif is_instruction(line):
            cur_rel_address += 1

    # second pass, parse instructions & calc relative offsets
    cur_rel_address = 0
    for instruction in (from_str(line) for line in program.split('\n') if is_instruction(line)):
            if instruction['op'] in FLOW_OPCODES:  # calc relative address to label
                label = instruction['offset']
                assert label in symbol_table
                instruction['offset'] = symbol_table[label] - cur_rel_address
            cur_rel_address += 1
            instructions.append(instruction)
    
    if ret_symbol_table:
        # also return symbol table for debug/ linkage purposes
        return instructions, symbol_table

    return instructions

