use super::{memory::Memory, operand::ComboOperand};

pub fn run_operation(opcode: u8, operand: u8, program_memory: &mut Memory) -> Option<u8> {
    let operation = match opcode {
        0 => adv,
        1 => bxl,
        2 => bst,
        3 => jnz,
        4 => bxc,
        5 => out,
        6 => bdv,
        7 => cdv,
        _ => panic!("Invalid opcode: {opcode}"),
    };

    operation(operand, program_memory)
}

#[inline(always)]
fn advance_ip(program_memory: &mut Memory) {
    program_memory.instruction_pointer += 2;
}

fn adv(operand: u8, program_memory: &mut Memory) -> Option<u8> {
    program_memory.registers.a >>= ComboOperand::from(operand).to_value(program_memory);
    advance_ip(program_memory);
    None
}

fn bxl(operand: u8, program_memory: &mut Memory) -> Option<u8> {
    program_memory.registers.b ^= operand as u64;
    advance_ip(program_memory);
    None
}

fn bst(operand: u8, program_memory: &mut Memory) -> Option<u8> {
    program_memory.registers.b = ComboOperand::from(operand).to_value(program_memory) % 8;
    advance_ip(program_memory);
    None
}

fn jnz(operand: u8, program_memory: &mut Memory) -> Option<u8> {
    if program_memory.registers.a == 0 {
        advance_ip(program_memory);
    } else {
        program_memory.instruction_pointer = operand as usize;
    }
    None
}

fn bxc(_: u8, program_memory: &mut Memory) -> Option<u8> {
    program_memory.registers.b ^= program_memory.registers.c;
    advance_ip(program_memory);
    None
}

fn out(operand: u8, program_memory: &mut Memory) -> Option<u8> {
    let value = (ComboOperand::from(operand).to_value(program_memory) % 8) as u8;
    advance_ip(program_memory);
    Some(value)
}

fn bdv(operand: u8, program_memory: &mut Memory) -> Option<u8> {
    program_memory.registers.b =
        program_memory.registers.a >> ComboOperand::from(operand).to_value(program_memory);
    advance_ip(program_memory);
    None
}

fn cdv(operand: u8, program_memory: &mut Memory) -> Option<u8> {
    program_memory.registers.c =
        program_memory.registers.a >> ComboOperand::from(operand).to_value(program_memory);
    advance_ip(program_memory);
    None
}
