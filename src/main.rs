use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use serde::{Deserialize, Serialize};
use crate::turing_machine::{Rule, TuringMachine};

mod errors;
mod turing_machine;

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Config {
    pub alphabet: String,
    pub rules: HashMap<char, HashMap<usize, Rule>>,
    pub memory: String
}

fn load_config<P: AsRef<std::path::Path>>(path: P) -> Result<Config, Box<dyn Error>> {
    let unparsed_json = fs::read_to_string(path)?;
    let config = serde_json::from_str(&unparsed_json)?;
    Ok(config)
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = load_config("./config.json")?;

    let memory = config.memory.chars().collect();
    let alphabet = config.alphabet.chars().collect();

    let mut tm = TuringMachine::new(memory, alphabet, config.rules);
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
