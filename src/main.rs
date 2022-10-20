use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;

use serde::{Deserialize, Serialize};

use crate::errors::IncorrectRuleError;
use crate::turing_machine::{Rule, TuringMachine};

mod errors;
mod turing_machine;

#[derive(Clone, Debug, Deserialize, Serialize)]
struct RawConfig {
    pub alphabet: String,
    pub rules: HashMap<char, HashMap<usize, String>>,
    pub memory: String,
    pub head_position: usize,
}

struct Config {
    pub alphabet: HashSet<char>,
    pub memory: Vec<char>,
    pub rules: HashMap<char, HashMap<usize, Rule>>,
    pub head_position: usize,
}

fn load_config<P: AsRef<std::path::Path>>(path: P) -> Result<Config, Box<dyn Error>> {
    let unparsed_json = fs::read_to_string(path)?;
    let raw_config: RawConfig = serde_json::from_str(&unparsed_json)?;

    let mut rules = HashMap::with_capacity(raw_config.memory.len());
    for (value, raw_inner_rules) in raw_config.rules.iter() {
        let mut inner_rules = HashMap::with_capacity(raw_inner_rules.len());
        for (state, rule) in raw_inner_rules {
            inner_rules.insert(*state, Rule::from_str(rule)?);
        }
        rules.insert(*value, inner_rules);
    }

    Ok(Config {
        alphabet: raw_config.alphabet.chars().collect(),
        memory: raw_config.memory.chars().collect(),
        rules,
        head_position: raw_config.head_position,
    })
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = load_config("./config.json")?;

    let mut tm = TuringMachine::new(
        config.memory,
        config.alphabet,
        config.rules,
        config.head_position,
    );
    println!("{}", tm);

    loop {
        tm.execute_step().unwrap();
        println!("{}", tm);

        if tm.is_finished() {
            println!("Turing machine has finished working");
            break;
        }
    }

    Ok(())
}
