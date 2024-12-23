use std::ops::{Deref, Index};

use super::register::Register;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Registers {
    pub a: u64,
    pub b: u64,
    pub c: u64,
}

impl Index<Register> for Registers {
    type Output = u64;

    fn index(&self, index: Register) -> &Self::Output {
        match index {
            Register::A => &self.a,
            Register::B => &self.b,
            Register::C => &self.c,
        }
    }
}

impl<T: Deref<Target = [u64]>> From<T> for Registers {
    fn from(value: T) -> Self {
        let &[a, b, c] = (&*value)
            .try_into()
            .expect("Slice must have exactly 3 elements");
        Registers { a, b, c }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Memory {
    pub registers: Registers,
    pub instruction_pointer: usize,
}
