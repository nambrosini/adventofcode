use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::io::{self, Read};
use std::str::FromStr;
use std::fs;

#[derive(Debug)]
struct CircuitError {
    reason: String,
}

impl CircuitError {
    fn new(reason: String) -> CircuitError {
        return CircuitError { reason };
    }
}

impl Error for CircuitError {}

impl fmt::Display for CircuitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CircuitError({})", self.reason)
    }
}

struct Circuit {
    registers: HashMap<String, CircuitValue>,
}

impl Circuit {
    fn get_key(&mut self, s: &str) -> Result<CircuitValue, Box<Error>> {
        let value = self
            .registers
            .get(s)
            .ok_or(format!("Missing a key: {}", s))?
            .clone();
        Ok(match value {
            CircuitValue::Value(v) => value.clone(),
            CircuitValue::ValueRule(_) => {
                let v = value.calculate(self)?;
                self.registers.insert(s.to_owned(), v);
                self.registers[s].clone()
            }
        })
    }

    fn get_val(&mut self, s: &str) -> Result<u16, Box<Error>> {
        return self.get_key(s)?.get_val();
    }
}

impl FromStr for Circuit {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut circ = Self {
            registers: HashMap::new(),
        };
        for line in s.split("\n") {
            let parts: Vec<&str> = line.split(" -> ").collect();
            let key = parts[1].to_owned();
            let rule = parts[0].parse()?;
            circ.registers.insert(key, CircuitValue::ValueRule(rule));
        }
        Ok(circ)
    }
}

#[derive(Clone)]
enum CircuitValue {
    ValueRule(Rule),
    Value(u16),
}

impl CircuitValue {
    fn calculate(&self, c: &mut Circuit) -> Result<CircuitValue, Box<Error>> {
        match self {
            CircuitValue::ValueRule(rule) => {
                Ok(CircuitValue::Value(rule.calculate(c)?))
            }
            CircuitValue::Value(v) => Ok(CircuitValue::Value(*v)),
        }
    }

    fn get_val(&self) -> Result<u16, Box<Error>> {
        match self {
            CircuitValue::Value(v) => Ok(*v),
            CircuitValue::ValueRule(r) => Err(Box::new(CircuitError::new(format!(
                "Should've been precalculated{:?}",
                r
            )))),
        }
    }
}

#[derive(Debug, Clone)]
enum Rule {
    Lshift(Value, Value),
    Rshift(Value, Value),
    And(Value, Value),
    Or(Value, Value),
    Not(Value),
    Direct(Value),
}

impl Rule {
    fn calculate(&self, c: &mut Circuit) -> Result<u16, Box<dyn Error>> {
        let val = match self {
            Rule::Lshift(key, shift) => key.get_val(c)? << shift.get_val(c)?,
            Rule::Rshift(key, shift) => key.get_val(c)? >> shift.get_val(c)?,
            Rule::And(key1, key2) => key1.get_val(c)? & key2.get_val(c)?,
            Rule::Or(key1, key2) => key1.get_val(c)? | key2.get_val(c)?,
            Rule::Not(key) => !key.get_val(c)?,
            Rule::Direct(key) => key.get_val(c)?,
        };
        Ok(val)
    }
}

impl FromStr for Rule {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("NOT") {
            let sp: Vec<&str> = s.split(" ").collect();
            return Ok(Rule::Not(sp[1].parse()?));
        }
        let sp: Vec<&str> = s.split(" ").collect();
        if sp.len() == 1 {
            return Ok(Rule::Direct(sp[0].parse()?));
        }
        match sp[1] {
            "LSHIFT" => Ok(Rule::Lshift(sp[0].parse()?, sp[2].parse()?)),
            "RSHIFT" => Ok(Rule::Rshift(sp[0].parse()?, sp[2].parse()?)),
            "AND" => Ok(Rule::And(sp[0].parse()?, sp[2].parse()?)),
            "OR" => Ok(Rule::Or(sp[0].parse()?, sp[2].parse()?)),
            _ => Err(Box::new(CircuitError::new(format!(
                "Can't parse operation from string {}",
                s
            )))),
        }
    }
}

#[derive(Debug, Clone)]
enum Value {
    Key(String),
    Value(u16),
}

impl Value {
    fn get_val(&self, c: &mut Circuit) -> Result<u16, Box<Error>> {
        match self {
            Value::Key(s) => c.get_val(&s),
            Value::Value(v) => Ok(*v),
        }
    }
}

impl FromStr for Value {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse() {
            Ok(v) => return Ok(Value::Value(v)),
            Err(_) => Ok(Value::Key(s.to_owned())),
        }
    }
}

fn main() {
    let mut input = fs::read_to_string("input.txt").unwrap();
    let mut circuit: Circuit = input.parse().unwrap();
    println!("A value is {}", circuit.get_val("a").unwrap());
    let mut circuit: Circuit = input.parse().unwrap();
    circuit.registers.insert(
        "b".to_owned(),
        CircuitValue::ValueRule(Rule::Direct(Value::Value(46065))),
    );
    println!("A value is {}", circuit.get_val("a").unwrap());
}