use crate::cli::RunCommand;
use anyhow::{Result};
use async_trait::async_trait;
use clap::Args;

#[derive(Args, Debug)]
pub struct Random {}

#[async_trait]
impl RunCommand for Random {
    async fn run(self) -> Result<()> {
        println!("random");
        Ok(())
    }
}
