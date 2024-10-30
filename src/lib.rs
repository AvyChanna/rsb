mod config;

use std::{fs, marker::PhantomData, net::SocketAddr, path::PathBuf, str::FromStr};

use clap::Parser;
use config::{DEFAULT_CLI_HELP_PATH, DEFAULT_SERVE_ADDR};
use rsb_schema::Resume;
use rsb_template::generate;

mod build {
	pub const LONG_HELP_TEXT: &str = include_str!(concat!(env!("OUT_DIR"), "/long-help.txt"));
}

#[derive(Debug, Parser)]
#[command(version, long_version=build::LONG_HELP_TEXT, about = "Create resumes from JsonResume structured data")]
pub enum Command {
	#[command(visible_alias = "gen", about = "generate resume from input")]
	Generate {
		#[arg(value_name = "INPUT_PATH", help = "file path for data", value_parser = PathBuf::from_str)]
		path: PathBuf,
	},
	#[command(visible_alias = "check", about = "check input for errors")]
	Validate {
		#[arg(value_name = "INPUT_PATH", help = "file path for data", value_parser=PathBuf::from_str)]
		path: PathBuf,
	},
	#[command(about = "start a server for easy editing")]
	Serve {
		#[arg(help = "bind address for the server", value_parser = SocketAddr::from_str, default_value_t = DEFAULT_SERVE_ADDR)]
		address: SocketAddr,
	},
	#[clap(hide = true)]
	GenerateMarkdownHelp {
		#[arg(value_parser=PathBuf::from_str, default_value=DEFAULT_CLI_HELP_PATH)]
		path: PathBuf,
	},
}

impl Command {
	pub fn handle_cmd(&self) -> anyhow::Result<()> {
		match self {
			Command::Generate { path, .. } => {
				log::debug!("Running generate with path: {:?}", path);
				let resume_data = Resume::from_file(path)?;
				log::debug!("found data: {}", resume_data);
				println!("{}", generate(resume_data)?);
				Ok(())
			}
			Command::Validate { path } => {
				log::debug!("Running validate with path: {:?}", path);
				let resume_data = Resume::from_file(path)?;
				log::debug!("found data: {}", resume_data);
				Ok(())
			}
			Command::GenerateMarkdownHelp { path } => {
				log::debug!("Running md_help_gen with out path: {:?}", path);
				let md_opts = clap_markdown::MarkdownOptions::new().show_footer(false);
				let md_str = clap_markdown::help_markdown_custom::<Self>(&md_opts);
				let parent = path
					.parent()
					.ok_or_else(|| anyhow::anyhow!("Could not find base path for file: {:?}", path))?;

				fs::create_dir_all(parent)?;
				Ok(fs::write(path, md_str)?)
			}
			Command::Serve { address } => {
				log::debug!("Running serve with address: {}", address);
				todo!("server handler"); // Add leptos and axum entrypoints here
			}
		}
	}
}
