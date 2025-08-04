use anyhow::Result;

pub trait KvEngine {
    fn set(&mut self, key: String, value: String) -> Result<()>;
    fn get(&self, key: String) -> Result<Option<String>>;
    fn delete(&mut self, key: String) -> Result<()>;
}
