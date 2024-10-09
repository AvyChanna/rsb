mod schema;
mod template;

use std::path::PathBuf;

pub use schema::ResumeSchema;
pub use template::TemplateOpts;

pub fn generate(path: PathBuf, opts: TemplateOpts) -> anyhow::Result<String> {
    let inp = ResumeSchema::try_from(path)?;
    log::debug!("ResumeData:{:#?}", inp);
    template::fill(inp, opts)
}

pub fn validate(path: PathBuf) -> anyhow::Result<()> {
    let inp = ResumeSchema::try_from(path)?;
    log::debug!("ResumeData:{:#?}", inp);
    Ok(())
}
