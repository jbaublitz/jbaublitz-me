extern crate web_sys;

use std::error::Error;
use std::fmt::{self,Display};

use wasm_bindgen::prelude::*;
use web_sys::console;

mod contact;
use contact::contact;

mod home;
use home::home;

mod humans;
use humans::humans;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Debug)]
struct JbaublitzError(&'static str);

impl Display for JbaublitzError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ERROR: {}", self.0)
    }
}

impl Error for JbaublitzError { }

#[wasm_bindgen]
pub fn choose_page(page: &str) {
    let res = match page {
        "humans" => humans(),
        "contact" => contact(),
        _ => home(),
    };
    res.unwrap_or_else(|e| {
        let error = format!("{}", e);
        console::log_1(&error.into());
    });
}
