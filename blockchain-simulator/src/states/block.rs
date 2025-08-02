use chrono::Utc;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use super::Transaction;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: u128,
    pub transactions: Vec<Transaction>,
    pub prev_block_hash: String,
    pub nonce: u64,
    pub hash: String,
}

impl Block {
    pub fn new(index: u64, transactions: Vec<Transaction>, prev_block_hash: String) -> Self {
        let timestamp = Utc::now().timestamp_millis() as u128;
        let (nonce, hash) = Self::mine_block(index, timestamp, &prev_block_hash, &transactions);

        Self {
            index,
            timestamp,
            transactions,
            prev_block_hash,
            nonce,
            hash,
        }
    }

    // this function helps us to find the hash by combining all the data from the block
    // with a suitable nounce
    pub fn mine_block(
        index: u64,
        timestamp: u128,
        prev_block_hash: &String,
        transactions: &[Transaction],
    ) -> (u64, String) {
        // difficulty runs the loops to increase the number of zero's in front of the hash
        let difficulty = 2;
        let mut nounce = 0;

        loop {
            let block_data = format!(
                "{} {} {:?} {} {}",
                index, timestamp, transactions, prev_block_hash, nounce
            );
            let hash = format!("{:x}", Sha256::digest(&block_data));
            if hash.starts_with(&"0".repeat(difficulty)) {
                return (nounce, hash);
            }
            nounce += 1
        }
    }

    pub fn verify_chain(chain: Vec<Block>) {
        let difficulty = 2;

        for i in 1..chain.len() {
            let current = &chain[i];
            let prev = &chain[i - 1];

            if current.prev_block_hash != prev.hash {
                println!("{}", current.prev_block_hash);
                println!("{}", prev.hash);
                println!("Invalid block link at {}", i);
                return;
            }

            let block_data = format!(
                "{} {} {:?} {} {}",
                current.index,
                current.timestamp,
                current.transactions,
                current.prev_block_hash,
                current.nonce
            );

            let computed_hash = format!("{:x}", Sha256::digest(&block_data));

            if current.hash != computed_hash {
                println!("Invalid block hash for {}", current.index);
                return;
            }

            if !current.hash.starts_with(&"0".repeat(difficulty)) {
                println!("POW is not satisfied for block {}", i);
                return;
            }
        }

        println!("chain is valid!");
        return;
    }
}
