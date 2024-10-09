mod resume_schema_impl;

use std::{fs::read_to_string, path::PathBuf};

use anyhow::Context;
use jsonnet::JsonnetVm;

pub use resume_schema_impl::*;

impl TryFrom<PathBuf> for ResumeSchema {
    type Error = anyhow::Error;
    fn try_from(path: PathBuf) -> anyhow::Result<Self> {
        from_file(path)
    }
}

enum FileType {
    Json5,
    Jsonnet,
    Yaml,
    Ron,
    Unknown(String),
}

fn extension_to_type(ext: &str) -> FileType {
    match ext {
        "json" | "json5" => FileType::Json5,
        "jsonnet" => FileType::Jsonnet,
        "yaml" | "yml" => FileType::Yaml,
        "ron" => FileType::Ron,
        _ => FileType::Unknown(ext.to_owned()),
    }
}

fn from_file(path: PathBuf) -> anyhow::Result<ResumeSchema> {
    let ext = path
        .extension()
        .with_context(|| "can not determine file extension type")?
        .to_str()
        .with_context(|| "error parsing file extension type")?;
    let inp_type = extension_to_type(ext);
    let inp = read_to_string(&path)?;
    let inp_str = inp.as_str();
    match inp_type {
        FileType::Json5 => Ok(json5::from_str(inp_str)?),
        FileType::Jsonnet => eval_jsonnet(path),
        FileType::Yaml => Ok(serde_yml::from_str(inp_str)?),
        FileType::Ron => Ok(ron::from_str(inp_str)?),
        FileType::Unknown(ext) => Err(anyhow::format_err!("Unrecognized file type: {}", ext)),
    }
}

fn eval_jsonnet(path: PathBuf) -> anyhow::Result<ResumeSchema> {
    let mut vm = JsonnetVm::new();
    vm.max_trace(Some(20));
    vm.max_stack(200);

    let output = vm.evaluate_file(path);
    match output {
        Ok(ref val) => log::debug!("jsonnet out:{}", val),
        Err(ref err) => log::error!("jsonnet err: {}", err),
    }
    match output {
        Ok(val) => Ok(json5::from_str(&val)?),
        Err(err) => anyhow::bail!("{}", err),
    }
}
