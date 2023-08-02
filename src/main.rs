mod cli;
pub mod commands;
mod model;
mod mp3_stream_decoder;
mod player;
mod utils;



use clap::Parser;

use rodio::{source::Source};













use crate::cli::Cli;




#[tokio::main]
async fn main() {
    Cli::parse().run().await;
}
