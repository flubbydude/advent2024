pub mod memory;

mod operand;
mod operation;
mod register;

use std::iter;

use memory::Memory;
use operation::run_operation;

pub fn run_program(
    tribit_code: &[u8],
    mut program_memory: Memory,
) -> impl IntoIterator<Item = u8> + '_ {
    iter::from_fn(move || {
        if program_memory.instruction_pointer >= tribit_code.len() - 1 {
            None
        } else {
            let opcode = tribit_code[program_memory.instruction_pointer];
            let operand = tribit_code[program_memory.instruction_pointer + 1];

            Some(run_operation(opcode, operand, &mut program_memory))
        }
    })
    .flatten()
}
