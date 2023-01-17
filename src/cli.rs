use adborc::command::commands::commands::Commands;
use adborc::command::commands::functions::{execute_command, setup_listener, setup_mangen};
use clap::Parser;

#[derive(Parser)]
#[clap(name="adborc", author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

impl Cli {
    pub fn process(self) {
        setup_mangen(self.command.clone());
        setup_listener(self.command.clone());
        execute_command(self.command.clone());
    }
}
