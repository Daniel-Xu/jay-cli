use crate::commands::random::Random;
use crate::commands::single::Single;
use anyhow::Result;
use async_trait::async_trait;
use clap::{Parser, Subcommand};
use colored::Colorize;
use std::process::ExitCode;

#[derive(Parser, Debug)]
pub struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Choose song manually one by one after playing
    Single(Single),

    /// Song automatically chosen for you after playing
    Random(Random),
}

#[async_trait]
pub trait RunCommand {
    async fn run(self) -> Result<()>;
}

impl Cli {
    pub async fn run(self) -> ExitCode {
        let output = match self.command {
            Commands::Single(single) => single.run().await,
            Commands::Random(random) => random.run().await,
        };

        match output {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("{}", e.context("failed to run command").to_string().red());
                ExitCode::FAILURE
            }
        }
    }
}
