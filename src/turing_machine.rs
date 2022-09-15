use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use crate::errors::{NoRuleError, TuringMachineError};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Direction {
    Left,
    Stay,
    Right,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Rule {
    pub direction: Direction,
    pub value: char,
    pub state: usize,
}

#[derive(Clone, Debug)]
pub struct TuringMachine {
    rules: HashMap<char, HashMap<usize, Rule>>,
    memory: Vec<char>,
    head: usize,
    state: usize,
    cycle: u64,
}

impl TuringMachine {
    pub fn new(
        memory: Vec<char>,
        alphabet: HashSet<char>,
        rules: HashMap<char, HashMap<usize, Rule>>,
    ) -> Self {
        if memory.is_empty() {
            panic!("memory_size cannot be empty")
        }

        let mut alphabet = alphabet;
        alphabet.insert('\\'); // Special symbol for empty memory cells

        let unknown_symbols: HashSet<char> = rules.keys()
            .copied()
            .chain(
                rules.values()
                    .flat_map(|rs| rs.values())
                    .map(|r| r.value)
            )
            .chain(memory.iter().copied())
            .filter(|k| !alphabet.contains(k))
            .collect();

        if !unknown_symbols.is_empty() {
            panic!("rules or memory contains unknown symbols: {:?}", unknown_symbols)
        }

        TuringMachine {
            memory,
            head: 0,
            state: 1,
            cycle: 0,
            rules,
        }
    }

    pub fn execute_step(&mut self) -> Result<(), TuringMachineError> {
        let value = self.memory[self.head];

        let rule = self.rules
            .get(&value)
            .and_then(|rs| rs.get(&self.state));

        if rule.is_none() {
            return Err(NoRuleError::wrapped(value, self.state));
        }

        let rule = *rule.unwrap();

        self.memory[self.head] = rule.value;

        match rule.direction {
            Direction::Left => self.move_left(),
            Direction::Stay => (),
            Direction::Right => self.move_right()
        }

        self.state = rule.state;
        self.cycle += 1;

        Ok(())
    }

    pub fn is_finished(&self) -> bool {
        self.state == 0
    }

    fn move_left(&mut self) {
        if self.head == 0 {
            self.head = self.memory.len() - 1
        } else {
            self.head -= 1
        }
    }

    fn move_right(&mut self) {
        if self.head == self.memory.len() - 1 {
            self.head = 0
        } else {
            self.head += 1
        }
    }

    fn memory_to_string(&self) -> String {
        let mut s = String::with_capacity(self.memory.len() + 2);

        for i in 0..self.head {
            s.push(self.memory[i]);
        }

        s.push('{');
        s.push(self.memory[self.head]);
        s.push('}');

        for i in (self.head + 1)..self.memory.len() {
            s.push(self.memory[i]);
        }

        s
    }
}

impl Display for TuringMachine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Cycle: {}, state: {}, head index: {}\nMemory dump: {}",
            self.cycle,
            self.state,
            self.head,
            self.memory_to_string()
        )
    }
}
