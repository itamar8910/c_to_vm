from cpu.instructions import from_str


def assemble(program):
    """
    assebles given program
    returns instructions
    """
    instructions = []
    # TODO: impl. two pass assembling to handle labels
    for line in program.split('\n'):
        if line.strip():
            instructions.append(from_str(line))
    return instructions

