use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NoRuleError {
    pub value: char,
    pub state: usize,
}

impl NoRuleError {
    pub fn new(value: char, state: usize) -> Self {
        NoRuleError {
            value,
            state,
        }
    }

    pub fn wrapped(value: char, state: usize) -> TuringMachineError {
        TuringMachineError::NoRule(Self::new(value, state))
    }
}

impl Display for NoRuleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "No rule for value {} and state {}", self.value, self.state)
    }
}

impl Error for NoRuleError {}

#[derive(Clone, Debug)]
pub enum TuringMachineError {
    NoRule(NoRuleError)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IncorrectRuleError {
    pub rule: String,
}

impl IncorrectRuleError {
    pub fn new(rule: String) -> Self {
        Self {
            rule
        }
    }
}

impl Display for IncorrectRuleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cannot parse rule `{}`", self.rule)
    }
}

impl Error for IncorrectRuleError {}
