mod model;

use anyhow::{anyhow, bail};
use std::{fs::read_to_string, path::PathBuf};

pub use model::*;

pub enum DataType {
	Json5,
	Yaml,
	#[cfg(feature = "ron")]
	Ron,
}

enum FileType {
	Json5,
	Yaml,
	#[cfg(feature = "ron")]
	Ron,
	#[cfg(feature = "jsonnet")]
	Jsonnet,
}

impl TryFrom<FileType> for DataType {
	type Error = anyhow::Error;
	fn try_from(value: FileType) -> anyhow::Result<Self> {
		match value {
			FileType::Json5 => Ok(DataType::Json5),
			FileType::Yaml => Ok(DataType::Yaml),
			#[cfg(feature = "ron")]
			FileType::Ron => Ok(DataType::Ron),
			#[cfg(feature = "jsonnet")]
			FileType::Jsonnet => bail!("Jsonnet can not be inited from buffer"),
		}
	}
}

trait PathFileType {
	fn file_type(&self) -> anyhow::Result<FileType>;
}

impl PathFileType for &PathBuf {
	fn file_type(&self) -> anyhow::Result<FileType> {
		let ext = self
			.extension()
			.and_then(|x| x.to_str())
			.ok_or(anyhow!("could not get extension"))?;

		match ext {
			"json" | "json5" => Ok(FileType::Json5),
			"yaml" | "yml" => Ok(FileType::Yaml),
			#[cfg(feature = "ron")]
			"ron" => Ok(FileType::Ron),
			#[cfg(feature = "jsonnet")]
			"jsonnet" => Ok(FileType::Jsonnet),
			_ => bail!("unknown extension {} for file {:?}", ext, self),
		}
	}
}

impl Resume {
	pub fn from_buffer(#[allow(unused_variables)] data: &str, file_type: DataType) -> anyhow::Result<Self> {
		match file_type {
			DataType::Json5 => Ok(json5::from_str(data)?),
			DataType::Yaml => Ok(serde_yml::from_str(data)?),
			#[cfg(feature = "ron")]
			DataType::Ron => Ok(ron::from_str(data)?),
		}
	}

	pub fn from_file(path: &PathBuf) -> anyhow::Result<Self> {
		let file_type = path.file_type()?;

		match file_type {
			#[cfg(feature = "jsonnet")]
			FileType::Jsonnet => {
				let mut vm = jsonnet::JsonnetVm::new();
				vm.max_trace(Some(20));
				vm.max_stack(200);

				let output = vm.evaluate_file(path);

				match output {
					Ok(val) => {
						log::debug!("jsonnet out:{}", val);
						Ok(json5::from_str(&val)?)
					}
					Err(err) => {
						log::error!("jsonnet err: {}", err);
						bail!("{}", err)
					}
				}
			}
			_ => {
				let inp = read_to_string(&path)?;
				let inp_str = inp.as_str();
				Resume::from_buffer(inp_str, DataType::try_from(file_type)?)
			}
		}
	}
}
