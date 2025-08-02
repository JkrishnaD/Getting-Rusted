use std::io::{self, Write};

use states::{Block, Transaction};

pub mod states;

fn main() {
    let mut mempool: Vec<Transaction> = vec![];
    let mut chain: Vec<Block> = vec![];

    println!("Simple Block Simulator!");
    println!(
        "Enter Transaction (format: sender reciever amount), type mine to create a block, type exit to quit"
    );

    loop {
        println!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();

        let trimmed = input.trim();

        if trimmed == "exit" {
            break;
        } else if trimmed == "mine" {
            if mempool.is_empty() {
                println!("No transactions to mine");
                continue;
            }
            println!("reached mine");
            let prev_hash = chain
                .last()
                .map_or(String::from("genesis"), |b| b.hash.clone());

            println!("Started mining....");
            let block = Block::new(chain.len() as u64, mempool.clone(), prev_hash);
            println!("Block mined {}", block.hash);
            chain.push(block);
            mempool.clear();
        } else if trimmed == "verify" {
            Block::verify_chain(chain.clone());
        } else {
            let parts: Vec<&str> = trimmed.split_whitespace().collect();

            if parts.len() != 3 {
                println!("Invalid Transaction use : sender reciever amount");
                continue;
            }

            let sender = parts[0].to_string();
            let reciever = parts[1].to_string();
            let amount = parts[2].parse::<u64>();

            match amount {
                Ok(a) => {
                    let tx = Transaction::new(sender, reciever, a);
                    println!("Trnasaction signature {}", tx);
                    mempool.push(tx);
                }
                Err(_) => println!("Transaction failed"),
            }
        }
    }

    println!("Final Blockchain:");
    for block in chain.iter() {
        println!(
            "Block #{} at [{}] with {} tx's -> hash:{}",
            block.index,
            block.timestamp,
            block.transactions.len(),
            block.hash
        )
    }
}
