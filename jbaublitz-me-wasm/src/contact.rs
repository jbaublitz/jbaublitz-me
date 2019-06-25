use std::error::Error;

use super::JbaublitzError;

pub fn contact() -> Result<(), Box<dyn Error>> {
    Err(Box::new(JbaublitzError("Not yet implemented")))
}

