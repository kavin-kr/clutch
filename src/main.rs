mod downloader;
mod indicator;

use anyhow::Result;
use clap::Parser;
use reqwest::Url;

/// A simple file downloader
#[derive(Debug, Parser)]
#[clap(version)]
struct CliArgs {
    /// Download url [use quoted-string notation]
    url: Url,

    /// Output file name with extension
    #[clap(short, long)]
    output: Option<String>,
}

fn main() -> Result<()> {
    let cli_args = CliArgs::parse();

    downloader::download_file(cli_args.url, cli_args.output)?;

    Ok(())
}
