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
        let difficulty = 1;
        let mut nounce = 0;

        loop {
            let block_data = format!(
                "{} {} {:?} {} {} ",
                index, timestamp, transactions, prev_block_hash, nounce
            );
            let hash = format!("{:x}", Sha256::digest(&block_data));
            if hash.starts_with(&"0".repeat(difficulty)) {
                return (nounce, hash);
            }
            nounce += 1
        }
    }
}
