use std::fmt;

use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub sender: String,
    pub reciever: String,
    pub amount: u64,
    pub timestamp: u128,
}

impl Transaction {
    pub fn new(sender: String, reciever: String, amount: u64) -> Self {
        let timestamp = Utc::now().timestamp_millis() as u128;

        Self {
            sender,
            reciever,
            amount,
            timestamp,
        }
    }
}

// to display a struct we need display trait
impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Tx from {} -> {} | amount {} | at: {}",
            self.sender, self.reciever, self.amount, self.timestamp
        )
    }
}
