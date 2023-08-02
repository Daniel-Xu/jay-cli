mod cli;
pub mod commands;
mod model;
mod mp3_stream_decoder;
mod player;
mod utils;
use clap::Parser;

use rodio::source::Source;

use crate::cli::Cli;
use crate::utils::show_jay_ascii;

#[tokio::main]
async fn main() {
    show_jay_ascii();

    Cli::parse().run().await;
}
