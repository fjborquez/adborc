use std::io::Write;
use crate::command::commands::commands::Commands;
use crate::command::mangen::generable::Generable;

pub struct SupplierMangen;

impl Generable for SupplierMangen {
    fn generate(command: &Commands, out_file: &mut dyn Write, path: &String) -> () {
        #[cfg(feature = "mangen")] {
         {}
         use command::mangen::functions::print_mangen;
         let mut subcommand: &Command = command.find_subcommand("supplier").unwrap();

         print_mangen(
             subcommand,
             String::from("subcommands"),
             String::from("Error writing Supplier subcommands:"),
             out_file,
             &path,
         );
        }
    }
}