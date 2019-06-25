use std::error::Error;

use web_sys::window;

use super::JbaublitzError;

pub fn set_text(text: &str) -> Result<(), Box<dyn Error>> {
    let elem_op = window().and_then(|w| w.document())
        .and_then(|d| d.get_element_by_id("center-panel"));
    let elem_centerpanel = match elem_op {
        Some(e) => e,
        None => {
            return Err(Box::new(JbaublitzError("No element with id=\"center-panel\" \
                                                       found")));
        }
    };
    elem_centerpanel.set_inner_html(text);

    Ok(())
}
