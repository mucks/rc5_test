use crate::error::{Error, Result};

pub struct KeySize(pub u32);

impl Default for KeySize {
    fn default() -> Self {
        Self(16)
    }
}

impl KeySize {
    pub fn new(size: u32) -> Result<KeySize> {
        if size > 2040 {
            Err(Error::KeySizeError("Key size is too large".to_string()))
        } else {
            Ok(KeySize(size))
        }
    }
}
