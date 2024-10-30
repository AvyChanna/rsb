use hypertext::{html_elements, maud, GlobalAttributes, RenderIterator, Renderable};
use rsb_schema::{Basics, BasicsProfilesItem};

struct SpanWrapper<R: Renderable>(R);
impl<R: Renderable> Renderable for SpanWrapper<R> {
	fn render_to(self, output: &mut String) {
		maud! {
			div .contact .centered {
				span .vertical-seperator {
					(self.0)
				}
			}
		}
		.render_to(output);
	}
}

struct EmailRenderer(String);
impl Renderable for EmailRenderer {
	fn render_to(self, output: &mut String) {
		let mut href = String::from("mailto:");
		href.push_str(self.0.as_str());

		SpanWrapper(maud!(
			a href=(href) {
				(self.0)
			}
		))
		.render_to(output);
	}
}

struct UrlRenderer(String);
impl Renderable for UrlRenderer {
	fn render_to(self, output: &mut String) {
		SpanWrapper(maud! {
			a href=(self.0.as_str()) {
				(self.0)
			}
		})
		.render_to(output);
	}
}

struct ImageRenderer(String);
impl Renderable for ImageRenderer {
	fn render_to(self, output: &mut String) {
		log::warn!("embedding an image is experimental. YMMV");
		maud! {
			div .contact .centered {
				img #basics-image alt="Profile image" src=(self.0);
			}
		}
		.render_to(output);
	}
}

struct ProfileItemRenderer(BasicsProfilesItem);
impl Renderable for ProfileItemRenderer {
	fn render_to(self, output: &mut String) {
		if self.0.username.is_some() {
			log::warn!("username ignored in profile {:?}", self.0);
		}

		let url = match &self.0.url {
			Some(s) => s,
			None => {
				log::error!("profile url empty for {:?}. Skipping", self.0);
				return;
			}
		};

		maud! {
			div .contact .centered {
				span .vertical-separator {
					a href=(url) {
						(url)
					}
				}
			}
		}
		.render_to(output);
	}
}

struct NameRenderer(String);
impl Renderable for NameRenderer {
	fn render_to(self, output: &mut String) {
		maud! {
			h1 #basics-name-heading {
				(self.0)
			}
		}
		.render_to(output);
	}
}

pub struct Renderer(pub Basics);
impl Renderable for Renderer {
	fn render_to(self, output: &mut String) {
		if self.0.image.is_some() {
			log::warn!("basics.image will be ignored")
		}

		maud! {
			section #basics {
				(self.0.name.map(NameRenderer))
				(self.0.label.map(SpanWrapper))
				(self.0.image.map(ImageRenderer))
				(self.0.email.map(EmailRenderer))
				(self.0.phone.map(SpanWrapper))
				(self.0.url.map(UrlRenderer))
				(self.0.summary.map(SpanWrapper))
				(self.0.profiles.into_iter().map(ProfileItemRenderer).render_all())
			}
		}
		.render_to(output);
	}
}
