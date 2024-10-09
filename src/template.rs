use super::ResumeSchema;

#[derive(Debug)]
pub struct TemplateOpts {}

pub fn fill(vals: ResumeSchema, opts: TemplateOpts) -> anyhow::Result<String> {
    log::debug!("Templating with opts:{:?} and vals:\n{:#?}", opts, vals);
    todo!("template fill");
}
