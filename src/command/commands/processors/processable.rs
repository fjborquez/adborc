use clap::Subcommand;
use crate::net::TCPClient;

pub trait Processable<T: Subcommand> {
    fn process(command: T, client: TCPClient) -> ();
}