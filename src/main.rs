use std::env;

use clap::Parser;
use rsb_cli::Command;

pub const LOG_ENV_KEY: &str = "RUST_LOG";
pub const DEFAULT_LOG_LEVEL: &str = "info";

fn main() -> anyhow::Result<()> {
	if env::var_os(LOG_ENV_KEY).is_none() {
		env::set_var(LOG_ENV_KEY, DEFAULT_LOG_LEVEL);
	}

	pretty_env_logger::init_timed();

	let args = Command::<rsb_template::SimpleTemplate>::parse();

	log::info!("using config {:#?}", args);
	args.handle_cmd()
}
