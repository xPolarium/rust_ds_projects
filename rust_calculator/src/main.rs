
use clap::Parser;
use std::{collections::HashMap, fmt};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Parser, Debug)]
#[command(version, author, about)]
struct Args {
    left_digit: i32,

    #[arg(value_parser = valid_operator)]
    operator: Operator,

    right_digit: i32
}

fn main() {
    let args = Args::parse();

    let operation_map: HashMap<Operator, &str> = HashMap::from([
            (Operator::Add, "+"), 
            (Operator::Subtract, "-"), 
            (Operator::Multiply, "*"), 
            (Operator::Divide, "/")
        ]);

    let left_digit = args.left_digit;
    let right_digit = args.right_digit;
    let operator = args.operator;

    let result = match operator {
        Operator::Add => left_digit + right_digit,
        Operator::Subtract => left_digit - right_digit,
        Operator::Multiply => left_digit * right_digit,
        Operator::Divide => {
            let mut denom = right_digit;
            if denom == 0 {
                denom = 1;
            }

            left_digit / denom
        }
    };

    let middle_digit = operation_map.get(&operator).unwrap().to_owned();
    println!("\n{left_digit} {middle_digit} {right_digit} = {result}");
}

fn valid_operator(s: &str) -> Result<Operator, String> {
    match s {
        "+" => Ok(Operator::Add),
        "-" => Ok(Operator::Subtract),
        "*" => Ok(Operator::Multiply),
        "/" => Ok(Operator::Divide),
        _ => Err(format!("{} is not a valid operator. Use +, -, *, or /.", s))
    }
}