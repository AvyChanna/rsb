mod basics;
// mod awards;
// mod certificates;
mod education;
// mod interests;
// mod languages;
// mod projects;
// mod publications;
// mod references;
// mod skills;
// mod volunteer;
// mod work;

use hypertext::{html_elements, maud, GlobalAttributes, Renderable};
use rsb_schema::Resume;

pub struct Body(pub Resume);
impl Renderable for Body {
	fn render_to(self, output: &mut String) {
		maud! {
			body {
				div #resume .container {
					div #basics-section .section {
						(basics::Renderer(self.0.basics))
					}
					div #education-section .section {
						(education::Renderer(self.0.education))
					}
					// div #work-section .section {
					// 	(self.0.work)
					// }
					// div #publications-section .section {
					// 	(self.0.publications)
					// }
					// div #projects-section .section {
					// 	(self.0.projects)
					// }
					// div #skills-section .section {
					// 	(self.0.skills)
					// }
					// div #awards-section .section {
					// 	(self.0.awards)
					// }
					// div #certificates-section .section {
					// 	(self.0.certificates)
					// }
					// div #volunteer-section .section {
					// 	(self.0.volunteer)
					// }
					// div #interests-section .section {
					// 	(self.0.interests)
					// }
					// div #languages-section .section {
					// 	(self.0.languages)
					// }
					// div #references-section .section {
					// 	(self.0.references)
					// }
				}
			}
		}
		.render_to(output);
	}
}
