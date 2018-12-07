
from os import path, listdir, mkdir
import subprocess
import sys
import shutil
from os import unlink

"""
given a path to a directory of c files
and a dst path for the tests directory
compiles & runs each c file to get return code
and organizes tests inputs & targets in dst directory
"""

c_files_dir = sys.argv[1]
dst_tests_dir = sys.argv[2]

if not path.isdir(dst_tests_dir):
    mkdir(dst_tests_dir)

if not path.isdir(path.join(dst_tests_dir, 'inputs')):
    mkdir(path.join(dst_tests_dir, 'inputs'))

if not path.isdir(path.join(dst_tests_dir, 'targets')):
    mkdir(path.join(dst_tests_dir, 'targets'))

for c_file in listdir(c_files_dir):
    subprocess.run(f'gcc {path.join(c_files_dir, c_file)}', shell=True)
    child = subprocess.Popen('./a.out', stdout=subprocess.PIPE, shell=True)
    _ = child.communicate()[0]
    retcode = child.returncode
    with open(path.join(dst_tests_dir, 'targets', c_file.replace('.c', '.txt')), 'w') as f:
        f.write(str(retcode))
    shutil.copy(path.join(c_files_dir, c_file), path.join(dst_tests_dir, 'inputs', c_file))

unlink('a.out')