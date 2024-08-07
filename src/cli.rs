use crate::commands::loop_one::LoopOne;
use crate::commands::random::Random;
use crate::commands::single::Single;

use anyhow::Result;
use async_trait::async_trait;
use clap::{Parser, Subcommand};
use colored::Colorize;
use std::process::ExitCode;

#[derive(Parser, Debug)]
#[command(author = "danielxu")]
#[command(version)]
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

    /// choose first and then loop forever
    LoopOne(LoopOne),
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
            Commands::LoopOne(loop_one) => loop_one.run().await,
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
