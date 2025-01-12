use std::{cell::RefCell, rc::Rc};

use enum_map::EnumMap;

use crate::instruction::Instruction;

pub struct InstructionTrie {
    map: EnumMap<Instruction, Option<Rc<RefCell<InstructionTrie>>>>,
    best_paths: Vec<Vec<Instruction>>,
}

impl InstructionTrie {
    pub fn new(best_paths: Vec<Vec<Instruction>>) -> Self {
        InstructionTrie {
            map: EnumMap::default(),
            best_paths,
        }
    }

    pub fn get_child(&self, instruction: &Instruction) -> Option<Rc<RefCell<InstructionTrie>>> {
        self.map[*instruction].as_ref().map(Rc::clone)
    }

    pub fn insert(&mut self, instruction: Instruction, paths: Vec<Vec<Instruction>>) {
        if self.map[instruction].is_some() {
            panic!("Inserting something into trie that exists");
        }

        let child_trie = Self::new(paths);
        self.map[instruction] = Some(Rc::new(RefCell::new(child_trie)));
    }

    pub fn best_paths(&self) -> &[Vec<Instruction>] {
        &self.best_paths
    }
}
