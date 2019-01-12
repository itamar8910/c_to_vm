# c_to_vm
A C compiler that targets a VM, written is Rust


### This project has 3 components:
- **Virtual Machine**:

  An emulation of a CPU that has 8 regiters, can execute instructions with 17 different opcodes, use memory, and perform IO with memory mapped registers.

- **C compiler**:

  A C compiler that targets the VM's instructions set.

  **list of compiler features**
    - Evaluate expressions
    - Local & global variables
    - Flow control: if/else & loops
    - Scopes
    - Functions
    - Arrays & structs
    - Pointers
    - C strings

  Includes a linker and a basic preprocessor.

  Lexing & Parsing is performed using [pycparser](https://github.com/eliben/pycparser).

- **Operating System**:

  Can load programs to memory, has an assembler and a assembly-level debugger. Offers a minimal libc with print functions and malloc & free implementation. 

#### TODO list:
- Improve preprocessor: Add #define, #ifdef, macros.
- Add typedef, check type validity at compile time.
- Ultimately the goal is to compile gnu libc
