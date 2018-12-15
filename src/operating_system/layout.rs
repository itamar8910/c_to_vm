/*
Memory layout:
0-499 os stuff:
    - memory mapped registers:
    - 200 COS - char out status
    - 201 COD - char out data
    - 202 CIS - char in status
    - 203 CID - char in data
    
    to write a char, write its ascii value to COD & then set COS to 1
    to read a char, set CIS to 1 & read ascii value from CID
500-999 data
1000-3999 code
4000-5999 heap
6000-9999 stack


Stack frame:
local vars...
-----------------
reg_save (callee save)
----------------
prev_BP
ret_addr
ret_val (can span multiple addresses)
--------------
arg1
arg2
arg3


Call convention:
Calling the function:
    Caller: 
        - pushes args on the stack in reverse order
        - pushes space for return value (callee does this because distance between BP & ret val must be constant for RET instructions)
        - CALL - pushes return address (= IP + 1),
                 pushes value of current bp & updates bp=sp+1
                 jumps to function
    Callee:
        - 
        - saves all registers whose value would get destroyed
        - can allocate local vars on the stack etc.
Returning from the function:
    Callee:
        - pushes return value to the stack
        - restores values of saved registers
        - 
        - RET - SP = BP + 1
                restores BP
                jump to returna addr
*/

pub const PROGRAM_INIT_ADDRESS: u32 = 1000;
pub const DATA_INIT_ADDRESS: u32 = 500;
pub const INIT_SP_ADDRESS: u32 = 9999;

// memory mapped registers for io
pub const COS : u32 = 200; // char out status
pub const COD : u32 = 201; // char out data
pub const CIS : u32 = 202; // char in status
pub const CID : u32 = 203; // char in data
