// rcli csv -i input.csv -o output.json --header -d ','
use clap::Parser;
use rcli::{CmdExector, Opts};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry().with(fmt::layer()).init();
    let opts: Opts = Opts::parse();
    opts.cmd.execute().await?;
    Ok(())
}
