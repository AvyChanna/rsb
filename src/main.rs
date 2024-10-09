use log;
use std::{
    env, fs,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::PathBuf,
    str::FromStr,
};

use clap::Parser;

const ENV_LOG: &str = "RUST_LOG";
const DEFAULT_CLI_HELP_PATH: &str = "docs/cli_help.md";
const DEFAULT_LOG_LEVEL: &str = "info";
const DEFAULT_SERVE_ADDR: SocketAddr = {
    let host = Ipv4Addr::new(127, 0, 0, 1);
    let port = 8080;
    SocketAddr::new(IpAddr::V4(host), port)
};

#[derive(Debug, Parser)]
#[command(
    version,
    about = "Create resumes from JsonResume structured data.\nInput type is infered from file extension.\n(Supports JSON/JSON5/JSONNET/YAML/RON)"
)]
enum Command {
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

fn handle_generate(path: PathBuf) -> anyhow::Result<()> {
    log::debug!("Running generate with path: {:?}", path);
    println!("{}", rsb::generate(path, rsb::TemplateOpts {})?);
    Ok(())
}

fn handle_validate(path: PathBuf) -> anyhow::Result<()> {
    log::debug!("Running validate with path: {:?}", path);
    rsb::validate(path)
}

fn handle_serve(address: SocketAddr) -> anyhow::Result<()> {
    log::debug!("Running serve with address: {}", address);
    todo!("server handler"); // Add leptos and axum entrypoints here
}

fn handle_md_help_gen(path: PathBuf) -> anyhow::Result<()> {
    log::debug!("Running md_help_gen with out path: {:?}", path);
    let md_opts = clap_markdown::MarkdownOptions::new().show_footer(false);
    let md_str = clap_markdown::help_markdown_custom::<Command>(&md_opts);
    Ok(fs::write(path, md_str)?)
}

fn main() -> anyhow::Result<()> {
    if env::var_os(ENV_LOG).is_none() {
        env::set_var(ENV_LOG, DEFAULT_LOG_LEVEL);
    }

    pretty_env_logger::init();

    let cmd = Command::parse();

    log::info!("using config {:#?}", cmd);
    match cmd {
        Command::Generate { path } => handle_generate(path),
        Command::Validate { path } => handle_validate(path),
        Command::Serve { address } => handle_serve(address),
        Command::GenerateMarkdownHelp { path } => handle_md_help_gen(path),
    }
}
