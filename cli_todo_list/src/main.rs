use std::io;
use std::collections::VecDeque;

enum Command {
    Add(String),
    Edit(String),
    Delete(String),

    Unknown {
        command: String,
        task: String
    }
}

impl From<(String, String)> for Command {
    fn from((command, task): (String, String)) -> Self {
        match command.as_str() {
            "add" => Self::Add(task),
            "edit" => Self::Edit(task),
            "delete" => Self::Delete(task),
            _ => Self::Unknown {command, task}
        }
    }
}

fn read_command(command: Command) {
    match command {
        Command::Add(task) => println!("Add command {:?}", task),
        Command::Edit(task) => println!("Edit command {:?}", task),
        Command::Delete(task) => println!("Delete command {:?}", task),

        Command::Unknown { command, task } => println!("Something else command {:?}, task {:?}", command, task),
    }
}

fn main() -> Result<(), String> {
    
    println!("Command Line Todo List!");

    loop {
        println!("Enter a valid command or type 'exit' to stop the program:");
        
        let mut input = String::new();
        
        io::stdin().read_line(&mut input).expect("Couldn't parse arguments.");

        let input = input.trim();

        if input == "exit" {
            println!("Shutting down...");
            break;
        }

        let mut tokens: VecDeque<&str> = input.split_whitespace().collect();

        let first_arg = if let Some(first_arg) = tokens.pop_front() {
            match first_arg.parse::<String>() {
                Ok(val) => val,
                Err(_) => return Err("Cannot read first argument.".to_string())
            }
        } else {
            return Err("Cannot read first argument.".to_string()) 
        };

        let second_arg = itertools::join(tokens.iter(), " ");
        
        let command: Command = (first_arg, second_arg).into();
        read_command(command);
    }

    Ok(())
}
