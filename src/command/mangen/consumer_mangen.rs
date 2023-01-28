use std::io::Write;
use crate::command::commands::commands::Commands;
use crate::command::mangen::generable::Generable;

pub struct ConsumerMangen {}

impl Generable for ConsumerMangen {
    fn generate(command: &Commands, out_file: &mut dyn Write, path: &String) -> () {
        #[cfg(feature = "mangen")] {
            use command::mangen::functions::print_mangen;
            let mut subcommand: &Command = command.find_subcommand("consumer").unwrap();

            print_mangen(
                subcommand,
                String::from("subcommands"),
                String::from("Error writing Consumer subcommands:"),
                out_file,
                &path,
            );
        }
    }
}