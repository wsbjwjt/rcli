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
