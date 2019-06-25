use std::error::Error;

use web_sys::window;

use super::JbaublitzError;

pub fn humans() -> Result<(), Box<dyn Error>> {
    let elem_op = window().and_then(|w| w.document())
        .and_then(|d| d.get_element_by_id("center-panel"));
    let elem = match elem_op {
        Some(e) => e,
        None => {
            return Err(Box::new(JbaublitzError("No element with id=\"center-panel\" \
                                                       found")));
        }
    };
    elem.set_inner_html(r#"
<div id="center-panel-inner">
    <div class="body">
        <div class="summary">
            <p>We often forget about the <b>human</b> component of software in the industry.</p>
        </div>
        <p>In an effort to challenge the notion that software is only about the tech, these are the things I try to bring to my collaborations:</p>
    </div>
    <ul class="left-list">
        <li>Communicate online like you would face to face</li>
        <li>Acknowledge that we're all still learning</li>
        <li>Diversity matters and starts with individual acceptance of difference</li>
        <li>When explaining, think back to when you were learning and explain it to yourself back then</li>
        <li>Empathy is crucial and should be practiced mutually during interactions with collaborators</li>
    </ul>
    <div class="body">
        <p>I work on a volunteer basis as part of a introductory systems education group at <a href="https://www.resilientcoders.org">Resilient Coders</a>. If you think diversity matters, click on the link and consider supporting an organization that's trying to make that happen in a real and actionable way!</p>
    </div>
</div>
"#);

    Ok(())
}

