use crate::market::request::{MarketMakerRequest, SysStateRequest};
use crate::net::TCPClient;
use crate::command::commands::marketmaker::subcommand::MarketMakerCommands;
use crate::command::network::functions::send_request;

pub fn process_marketmaker_command(command: MarketMakerCommands, client: TCPClient) -> () {
    match command {
        MarketMakerCommands::Status => {
            let response = send_request(MarketMakerRequest::Status, &client);
            println!("{}", response);
        }
        MarketMakerCommands::Start => {
            let response = send_request(SysStateRequest::StartMarketMaker, &client);
            println!("{}", response);
        }
        MarketMakerCommands::Stop => {
            let response = send_request(SysStateRequest::StopMarketMaker, &client);
            println!("{}", response);
        }
        MarketMakerCommands::UseWhitelist => {
            let response = send_request(MarketMakerRequest::UseWhitelist, &client);
            println!("{}", response);
        }
        MarketMakerCommands::ResetWhitelist => {
            let response = send_request(MarketMakerRequest::ResetWhitelist, &client);
            println!("{}", response);
        }
        MarketMakerCommands::AddSupplier { peer_id } => {
            let response = send_request(
                MarketMakerRequest::WhitelistSupplier { key: peer_id.to_owned() },
                &client,
            );
            println!("{}", response);
        }
        MarketMakerCommands::RemoveSupplier { peer_id } => {
            let response = send_request(
                MarketMakerRequest::UnwhitelistSupplier { key: peer_id.to_owned() },
                &client,
            );
            println!("{}", response);
        }
        MarketMakerCommands::AddConsumer { peer_id } => {
            let response = send_request(
                MarketMakerRequest::WhitelistConsumer { key: peer_id.to_owned() },
                &client,
            );
            println!("{}", response);
        }
        MarketMakerCommands::RemoveConsumer { peer_id } => {
            let response = send_request(
                MarketMakerRequest::UnwhitelistConsumer { key: peer_id.to_owned() },
                &client,
            );
            println!("{}", response);
        }
    }
}
