use clap::{Parser, Subcommand};

/// CLI entry point
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Extrack {
    #[command(subcommand)]
    action: Action,
}

impl Extrack {
    pub fn action(&self) -> &Action {
        &self.action
    }
}

#[derive(Subcommand, Debug)]
pub enum Action {
    Track,
}
