use crate::{process_http_serve, CmdExector};

use super::verify_path;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub enum HttpSubCommand {
    #[command(about = "Serve a directory over HTTP")]
    Serve(HttpServeOpts),
}

#[derive(Parser, Debug)]
pub struct HttpServeOpts {
    /// The directory to serve
    #[arg(short, long, value_parser=verify_path, default_value=".")]
    pub dir: PathBuf,

    /// The port to serve on
    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,
}

impl CmdExector for HttpServeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        process_http_serve(self.dir, self.port).await
    }
}

impl CmdExector for HttpSubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            HttpSubCommand::Serve(opts) => opts.execute().await,
        }
    }
}
