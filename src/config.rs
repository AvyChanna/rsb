use std::net::{IpAddr, Ipv4Addr, SocketAddr};

pub const DEFAULT_CLI_HELP_PATH: &str = "docs/cli_help.md";
pub const DEFAULT_SERVE_ADDR: SocketAddr = {
	let host = Ipv4Addr::new(127, 0, 0, 1);
	let port = 8080;
	SocketAddr::new(IpAddr::V4(host), port)
};

