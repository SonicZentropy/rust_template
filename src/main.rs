#[allow(unused_imports)]
use tracing::{error, info, warn, debug, trace, Level };

fn main() -> anyhow::Result<()> {
	init();

	Ok(())
}

fn init() {
	tracing_subscriber::fmt()
		.with_max_level(Level::INFO)
		.without_time()
		.pretty()
		.init();

	trace!("Initialization Complete");
}
