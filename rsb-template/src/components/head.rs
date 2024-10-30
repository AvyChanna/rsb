use hypertext::{html_elements, maud, Raw, Renderable};
const CSS_STR: &str = include_str!("style.css");

pub struct Head(pub Option<String>);
impl Renderable for Head {
	fn render_to(self, output: &mut String) {
		let name = self.0.clone().unwrap_or_else(|| String::from("Resume"));
		let date = chrono::Utc::now().format("%Y-%m-%d").to_string();

		maud! {
			head {
				meta charset="utf-8";
				meta name="viewport" content="width=device-width, user-scalable=no, minimal-ui";
				title {
					(name) " - " (date)
				}
				// TODO: unsafe: preescaping is not safe. Validate css before doing this.
				// Leaving this because this is static css (and ran locally)
				style {
					(Raw(CSS_STR))
				}
			}
		}
		.render_to(output);
	}
}
