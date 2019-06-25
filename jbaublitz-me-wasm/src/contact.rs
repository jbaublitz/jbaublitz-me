use std::error::Error;

use web_sys::window;

use super::JbaublitzError;

pub fn contact() -> Result<(), Box<dyn Error>> {
    let elem_op = window().and_then(|w| w.document())
        .and_then(|d| d.get_element_by_id("center-panel"));
    let elem_centerpanel = match elem_op {
        Some(e) => e,
        None => {
            return Err(Box::new(JbaublitzError("No element with id=\"center-panel\" \
                                                       found")));
        }
    };
    elem_centerpanel.set_inner_html(r#"
<div id="center-panel-inner">
    <div class="contact-type">
        <p><b>Freenode IRC handle:</b></p>
    </div>
    <div class="contact-info">
        <p>jbaublitzzz</p>
    </div>
    <div class="contact-type">
        <p><b>Email:</b></p>
    </div>
    <div class="contact-info">
        <p>john.m.baublitz@gmail.com</p>
    </div>
    <div class="contact-type">
        <p><b>Github:</b></p>
    </div>
    <div class="contact-info">
        <p><a href="https://github.com/jbaublitz">https://github.com/jbaublitz</a></p>
    </div>
</div>
"#);

    Ok(())
}

