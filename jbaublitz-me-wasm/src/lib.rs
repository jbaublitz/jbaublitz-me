extern crate web_sys;

use std::error::Error;
use std::fmt::{self,Display};

use wasm_bindgen::prelude::*;
use web_sys::console;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[derive(Debug)]
struct JbaublitzError(&'static str);

impl Display for JbaublitzError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ERROR: {}", self.0)
    }
}

impl Error for JbaublitzError {

}

#[wasm_bindgen]
pub fn choose_page(page: &str) {
    match page {
        "index" => index().unwrap_or_else(|e| {
            let error = format!("{}", e);
            console::log_1(&error.into());
        }),
        "humans" => humans().unwrap_or_else(|e| {
            let error = format!("{}", e);
            console::log_1(&error.into());
        }),
        "contact" => contact().unwrap_or_else(|e| {
            let error = format!("{}", e);
            console::log_1(&error.into());
        }),
        _ => index().unwrap_or_else(|e| {
            let error = format!("{}", e);
            console::log_1(&error.into());
        }),
    };
}

fn index() -> Result<(), Box<dyn Error>> {
    Err(Box::new(JbaublitzError("OOPS")))
}

fn humans() -> Result<(), Box<dyn Error>> {
    Ok(())
}

fn contact() -> Result<(), Box<dyn Error>> {
    Ok(())
}
