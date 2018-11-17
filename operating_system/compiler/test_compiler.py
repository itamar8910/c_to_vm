



import pytest
from os import path, listdir

from operating_system.compiler import compiler
from operating_system.assembler import assemble
from operating_system.os import run_program

TESTS_DIR = 'operating_system/compiler/test_data'

def get_test_categories():
    return {
        category: {
            'inputs': sorted(listdir(path.join(TESTS_DIR, category, 'inputs'))),
            'targets': sorted(listdir(path.join(TESTS_DIR, category, 'targets'))),
        } for category in listdir(TESTS_DIR)
    }


test_cases = get_test_categories()

categories, inputs, targets = zip(*[(category, inp, tar) for category, data in test_cases.items() for inp, tar in zip(data['inputs'], data['targets'])])
assert all([i.replace('.c', '') == t.replace('.txt', '') for i, t in zip(inputs, targets)]), 'input/target mismatch!'

# we're unzipping & then zipping, but I thing it's clearer this way 

@pytest.mark.parametrize('category,input_f,tar_f', zip(categories, inputs, targets))
def test_single_case(category, input_f, tar_f):
    with open(path.join(TESTS_DIR, category, 'inputs', input_f)) as f:
        text = f.read()
    with open(path.join(TESTS_DIR, category, 'targets', tar_f)) as f:
        tar_val = int(f.read().strip())

    code = compiler.compile(text)
    instructions = assemble('\n'.join(code))
    proram_res = run_program(instructions)
    assert proram_res == tar_val