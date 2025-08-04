use std::io::{self, Write};

use crate::engine::{KvEngine, SimpleEngine};

pub mod cli;
pub mod config;
pub mod engine;
pub mod types;

fn main() {
    let mut engine = SimpleEngine::open("logs/wal.log").unwrap();

    println!("Welcome to kvdb! Type 'help' for commands. Type 'exit' to quit.");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();

        let trimmed = input.trim();

        if trimmed == "exit" {
            break;
        } else if trimmed == "help" {
            println!("Available commands:");
            println!("set <key> <value>");
            println!("get <key>");
            println!("del <key>");
            println!("exit");
            continue;
        };

        let mut parts = input.split_whitespace();
        let command = parts.next();
        let key = parts.next();
        let value = parts.next();

        match command {
            Some("set") => {
                if let (Some(k), Some(v)) = (key, value) {
                    let _ = engine.set(k.to_string(), v.to_string());
                    println!("Value set")
                } else {
                    println!("Enter the correct command (set key value)")
                }
            }
            Some("get") => {
                if let Some(k) = key {
                    match engine.get(k.to_string()).unwrap() {
                        Some(val) => println!("key-values is {} : {}", k, val),
                        None => println!("Key doesn't exist"),
                    }
                } else {
                    println!("Enter the key to get value")
                }
            }
            Some("del") => {
                if let Some(k) = key {
                    let _ = engine.delete(k.to_string());
                    println!("Deleted key {}", k)
                } else {
                    println!("Enter element key to delete")
                }
            }
            _ => println!("Invalid command entered! type help for commands"),
        }
    }
}
