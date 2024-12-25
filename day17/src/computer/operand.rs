use super::{memory::Memory, register::Register};

pub enum ComboOperand {
    Value(u8),
    Register(Register),
}

impl From<u8> for ComboOperand {
    fn from(value: u8) -> Self {
        match value {
            0..=3 => ComboOperand::Value(value),
            4 => ComboOperand::Register(Register::A),
            5 => ComboOperand::Register(Register::B),
            6 => ComboOperand::Register(Register::C),
            _ => panic!("Cannot convert {value} to ComboOperand"),
        }
    }
}

impl ComboOperand {
    pub fn to_value(&self, program_memory: &Memory) -> u64 {
        match *self {
            ComboOperand::Value(value) => value as u64,
            ComboOperand::Register(register) => program_memory.registers[register],
        }
    }
}
