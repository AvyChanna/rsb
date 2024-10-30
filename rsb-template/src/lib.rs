mod components;

use hypertext::{html_elements, maud, Renderable};
use rsb_schema::Resume;

pub fn generate(val: Resume) -> anyhow::Result<String> {
	log::debug!("Templating with values:\n{:#?}", val);
	let mut res = String::new();
	maud! {
		!DOCTYPE
		html {
			(components::Head(val.basics.name.clone()))
			(components::Body(val))
		}
	}
	.render_to(&mut res);
	Ok(res)
}
