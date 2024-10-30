use hypertext::{html_elements, maud, GlobalAttributes, RenderIterator, Renderable};
use rsb_schema::{EducationItem, Iso8601};

struct YearOnlyDateRenderer(Iso8601);
impl Renderable for YearOnlyDateRenderer {
	fn render_to(self, output: &mut String) {
		let year = match self.0 {
			Iso8601::Full(y, ..) => y,
			Iso8601::Year(y) => y,
			Iso8601::YearMonth(y, _) => y,
		};

		year.render_to(output);
	}
}

struct EducationItemRenderer(EducationItem);
impl Renderable for EducationItemRenderer {
	fn render_to(self, output: &mut String) {
		let study_type = match self.0.study_type {
			Some(ref a) => a,
			None => {
				log::warn!("No studyType in EducationItem {:?}. Skipping render", self.0);
				return;
			}
		};

		let area = match self.0.area {
			Some(ref a) => a,
			None => {
				log::warn!("No area in EducationItem {:?}. Skipping render", self.0);
				return;
			}
		};

		let institution = match self.0.institution {
			Some(ref a) => a,
			None => {
				log::warn!("No institution in EducationItem {:?}. Skipping render", self.0);
				return;
			}
		};

		if self.0.url.is_some() {
			log::warn!("Ignoring url in EducationItem {:?}", self.0);
		}
		if self.0.start_date.is_some() {
			log::warn!("Ignoring startDate in EducationItem {:?}", self.0);
		}
		if self.0.score.is_some() {
			log::warn!("Ignoring score in EducationItem {:?}", self.0);
		}

		if self.0.courses.len() != 0 {
			log::warn!("Ignoring courses in EducationItem {:?}", self.0);
		}
		maud! {
			li {
				(study_type) " in " (area) " from " (institution)
				@if let Some(end_date) = self.0.end_date {
					" (" (YearOnlyDateRenderer(end_date)) ")"
				}
			}
		}
		.render_to(output);
	}
}

pub struct Renderer(pub Vec<EducationItem>);
impl Renderable for Renderer {
	fn render_to(self, output: &mut String) {
		maud! {
			section #education {
				ul {
					(self.0.into_iter().map(EducationItemRenderer).render_all())
				}
			}
		}
		.render_to(output);
	}
}
