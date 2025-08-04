use std::{
    collections::HashMap,
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, BufWriter, Write},
    path::Path,
};

use anyhow::Result;

use crate::engine::KvEngine;

pub struct SimpleEngine {
    pub store: HashMap<String, String>, // in-memory key-value store
    pub wal: BufWriter<File>,           // write-ahead log for durability while app crashes
}

impl SimpleEngine {
    pub fn open(path: &str) -> Result<Self> {
        let wal_path = Path::new(path);

        // Ensure the WAL directory exists
        if let Some(parent) = wal_path.parent() {
            fs::create_dir_all(parent)?;
        }

        // Open the WAL file for append + read
        let wal_file = OpenOptions::new()
            .create(true)
            .append(true)
            .read(true)
            .open(wal_path)?;

        // Clone the file so we can read and write
        let reader = BufReader::new(wal_file.try_clone()?);

        // Rebuild the in-memory store from WAL
        let mut store = HashMap::new();
        for line in reader.lines() {
            let line = line?;
            if line.starts_with("SET ") {
                let mut parts = line[4..].splitn(2, ' ');
                if let (Some(k), Some(v)) = (parts.next(), parts.next()) {
                    store.insert(k.to_string(), v.to_string());
                }
            } else if line.starts_with("DEL ") {
                let key = &line[4..];
                store.remove(key);
            }
        }

        // Wrap the original file in a BufWriter for writing
        Ok(SimpleEngine {
            store,
            wal: BufWriter::new(wal_file),
        })
    }
}

impl KvEngine for SimpleEngine {
    // get the values and set the values in the file
    fn set(&mut self, key: String, value: String) -> Result<()> {
        writeln!(self.wal, "SET {} {}", key, value)?;
        self.wal.flush()?;
        self.store.insert(key, value);
        Ok(())
    }

    // gets the related data from the file
    fn get(&self, key: String) -> Result<Option<String>> {
        Ok(self.store.get(&key).cloned())
    }

    // delets the data regarding with the key in the file
    fn delete(&mut self, key: String) -> Result<()> {
        writeln!(self.wal, "DEL {}", key)?;
        self.wal.flush()?;
        self.store.remove(&key);
        Ok(())
    }
}
