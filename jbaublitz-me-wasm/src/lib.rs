extern crate web_sys;

use std::error::Error;
use std::fmt::{self,Display};

use wasm_bindgen::prelude::*;
use web_sys::console;

mod contact;
use contact::CONTACT;

mod graphics;
use graphics::graphics;

mod home;
use home::HOME;

mod humans;
use humans::HUMANS;

mod textset;
use textset::set_text;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Debug)]
pub struct JbaublitzError(&'static str);

impl Display for JbaublitzError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ERROR: {}", self.0)
    }
}

impl Error for JbaublitzError { }

#[wasm_bindgen]
pub fn choose_page(page: &str) {
    let res = match page {
        "humans" => set_text(HUMANS),
        "contact" => set_text(CONTACT),
        "graphics" => graphics(),
        _ => set_text(HOME),
    };
    res.unwrap_or_else(|e| {
        let error = format!("{}", e);
        console::log_1(&error.into());
    });
}
