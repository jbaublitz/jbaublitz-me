use std::error::Error;

use web_sys::window;

use super::JbaublitzError;

pub fn home() -> Result<(), Box<dyn Error>> {
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
	<div class="header header-one">Hello and thanks for dropping by!</div>
	<div class="header header-two">I'm John Baublitz and I do a lot of open source work.</div>
	<div class="header header-two">A little bit about me and my interests:</div>
	<div class="body">
		<p>I have been hacking on things since 2011. Feel free to poke around my Github for my open source personal projects and contributions. Most of my time is divided between contributing to large open source projects and maintaining my own personal projects in the hopes that they may be useful to someone along the way</p>
		<p>Some of the areas I enjoy focusing on most are:</p>
		<div class="center-list">
			<ul>
				<li>strong typing</li>
				<li>working with and writing code for networking and network stacks</li>
				<li>digging into compilers and programming languages</li>
				<li>working with operating systems both in kernel space and user space</li>
				<li>learning more about hardware</li>
				<li>systems security</li>
				<li>sometimes writing tooling to make life easier for myself</li>
			<ul>
		</div>
		<p>Because of the domains I work in, I've gravitated mostly towards C, Python, and Rust to get things done.</p>
	</div>
</div>
"#);

    Ok(())
}
