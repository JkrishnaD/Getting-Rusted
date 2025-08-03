# 🧠 kvdb — A Simple Persistent Key-Value Store in Rust

`kvdb` is a lightweight, CLI-based key-value database written in Rust. It is designed for learning and experimentation.

Inspired by LevelDB and RocksDB, `kvdb` starts with a simple in-memory map + Write-Ahead Log (WAL) for durability and evolves into an LSM-tree-based persistent store.

- It uses a `HashMap` for fast in-memory operations and persists data to disk using a Write-Ahead Log (WAL) for crash recovery.
- The CLI interface allows basic commands like `set`, `get`, and `delete`.
- The data is stored in a `.kvdb/` directory, making it easy to manage.
- The architecture is modular, allowing for future extensions like SSTables, compaction, and more.

---

## Features

- Fast in-memory reads and writes (via `HashMap`)
- Write-Ahead Logging for crash recovery
- Clean CLI interface for basic commands
- Data persisted on disk in `.kvdb/` directory
- Modular design (can be extended into SSTables, compaction, and more)

---

## How It Works

- **SET key value**
  Stores the key-value pair in memory and appends it to the WAL file.

- **GET key**
  Fetches the value from memory (or eventually SSTables).

- **DELETE key**
  Marks the key as deleted and appends a tombstone to the WAL.

- **Recovery**
  On startup, the engine replays the WAL to reconstruct the memtable.



## File Structure

```
kvdb/
├── src/
│   ├── main.rs                # CLI entry point
│   ├── cli.rs                 # Parser and command handling
│   ├── config.rs              # Configuration Loader
│   ├── types.rs               # Common Data types and helpers
│   ├── engine/                # Storage Engine Implementations
│   │   ├── mod.rs            # Engine module entry point
│   │   ├── kv_engine.rs      # Key-Value Engine Implementation
│   │   ├── wal_engine.rs     # Write-Ahead Log Engine Implementation
│   │   └── simple_engine.rs  # HashMap and Wal Implementation
│   ├── storage/              # File helpers and manifest logic
```

## Usage

#### Add a key

`./kvdb set name "Jaya Krishna"`

#### Retrieve a key

`./kvdb get name > "Jaya Krishna"`

#### Delete a key

`./kvdb delete name`

#### Try to get again

`./kvdb get name > null`
