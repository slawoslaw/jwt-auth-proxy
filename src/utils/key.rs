use std::fs;

pub trait PrivateKeyProvider {
    fn load(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>>;
}

pub trait PublicKeyProvider {
    fn load(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>>;
}

// LocalFileKeyLoader implementation for loading the keys from file
pub struct LocalFileKeyLoader {
    pub key_path: String,
}

impl PrivateKeyProvider for LocalFileKeyLoader {
    fn load(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let key_bytes = fs::read(&self.key_path)?;
        Ok(key_bytes)
    }
}

impl PublicKeyProvider for LocalFileKeyLoader {
    fn load(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let key_bytes = fs::read(&self.key_path)?;
        Ok(key_bytes)
    }
}
