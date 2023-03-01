use std::env::args;
use std::fmt;
use std::process;

fn main() {
    let config = Config::build(args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments:{err}");
        process::exit(1);
    });

    let output = run(config).unwrap_or_else(|err| {
        eprintln!("Application error: {err}");
        process::exit(1);
    });
    println!("Output: {output}")
}

enum Operation {
    Add,
    Subtract,
    Multlipy,
    Divide,
}

enum ConfigError {
    InvalidOperation(String),
    ParseIntError(String),
    NoArgument(String),
}

enum ApplicationError {
    DivideByZero,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConfigError::ParseIntError(x) => {
                write!(f, "Failed to parse {}", x)
            }
            ConfigError::InvalidOperation(x) => {
                write!(f, "Invalid Operation {}", x)
            }
            ConfigError::NoArgument(x) => {
                write!(f, "Failed to get argument {}", x)
            } // ConfigError::DivideByZero => {
              //     write!(f, "Dividing by zero")
              // }
        }
    }
}

impl fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ApplicationError::DivideByZero => {
                write!(f, "Dividing by zero")
            }
        }
    }
}

struct Config {
    n1: f64,
    n2: f64,
    operation: Operation,
}

impl Operation {
    fn new(operation: &str) -> Result<Operation, ConfigError> {
        match operation {
            "+" => Ok(Operation::Add),
            "-" => Ok(Operation::Subtract),
            "*" => Ok(Operation::Multlipy),
            "/" => Ok(Operation::Divide),
            opr => Err(ConfigError::InvalidOperation(opr.to_string())),
        }
    }
}

impl Config {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, ConfigError> {
        args.next();
        let n1 = match args.next() {
            Some(val) => val,
            None => return Err(ConfigError::NoArgument("argument 1".to_string())),
        };
        let n1 = match n1.parse::<f64>() {
            Ok(n) => n,
            Err(_) => return Err(ConfigError::ParseIntError(n1)),
        };

        let operation = match args.next() {
            Some(val) => val,
            None => return Err(ConfigError::NoArgument("operation".to_string())),
        };
        let operation = match Operation::new(&operation) {
            Ok(opr) => opr,
            Err(e) => return Err(e),
        };

        let n2 = match args.next() {
            Some(val) => val,
            None => return Err(ConfigError::NoArgument("argument 2".to_string())),
        };
        let n2 = match n2.parse::<f64>() {
            Ok(n) => n,
            Err(_) => return Err(ConfigError::ParseIntError(n2)),
        };

        Ok(Config { n1, n2, operation })
    }
}

fn run(config: Config) -> Result<f64, ApplicationError> {
    let n1 = config.n1;
    let n2 = config.n2;
    let opr = config.operation;
    match opr {
        Operation::Add => Ok(n1 + n2),
        Operation::Divide => {
            if n2 == 0.0 {
                Err(ApplicationError::DivideByZero)
            } else {
                Ok(n1 / n2)
            }
        }
        Operation::Multlipy => Ok(n1 * n2),
        Operation::Subtract => Ok(n1 - n2),
    }
}
