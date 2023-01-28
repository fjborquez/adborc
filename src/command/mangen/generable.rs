use std::io::Write;
use crate::command::commands::commands::Commands;

pub trait Generable {
    fn generate(command: &Commands, out_file: &mut dyn Write, path: &String) -> ();
}