use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone, Debug)]
enum Direction {
    Left,
    Stay,
    Right,
}

#[derive(Copy, Clone, Debug)]
struct Rule {
    direction: Direction,
    value: char,
    state: usize,
}

#[derive(Clone, Debug)]
struct TuringMachine {
    alphabet: HashSet<char>,
    rules: HashMap<char, Vec<Rule>>,
    memory: Vec<char>,
    head: usize,
    state: usize,
}

impl TuringMachine {
    pub fn new(memory_size: usize, alphabet: HashSet<char>, rules: HashMap<char, Vec<Rule>>) -> Self {
        if alphabet.contains(&'\\') {
            panic!("alphabet should not contain '\\' because '\\' is a special character")
        }

        if memory_size == 0 {
            panic!("memory_size cannot be empty")
        }

        TuringMachine {
            memory: vec!['\\'; memory_size],
            head: 0,
            state: 1,
            alphabet,
            rules,
        }
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

    fn execute(&mut self) -> Result<(), String> {
        let rule = self.rules
            .get(&self.memory[self.head])
            .map(|rs| rs[self.state]);

        if rule.is_none() {
            return format!("No rule for ");
        }
    }
}

fn main() {
    println!("Hello, world!");
}
