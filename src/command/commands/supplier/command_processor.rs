use crate::command::commands::supplier::subcommand::SupplierCommands;
use crate::market::request::{SupplierRequest, SysStateRequest};
use crate::net::TCPClient;
use crate::command::network::functions::send_request;

pub fn process_supplier_command(command: SupplierCommands, client: TCPClient) -> () {
    match command {
        SupplierCommands::Status => {
            let response = send_request(SupplierRequest::Status, &client);
            println!("{}", response);
        }
        SupplierCommands::Start {
            remote,
            port,
            user,
            secure,
        } => {
            let response = send_request(
                SysStateRequest::StartSupplier {
                    mm_host: remote.to_owned(),
                    mm_port: port.to_owned(),
                    name: user.to_owned(),
                    secure_comms: secure.to_owned(),
                },
                &client,
            );
            println!("{}", response);
        }
        SupplierCommands::Stop => {
            let response = send_request(SysStateRequest::StopSupplier, &client);
            println!("{}", response);
        }
        SupplierCommands::Supply { devices } => {
            let response = send_request(SupplierRequest::SupplyDevices { devices }, &client);
            println!("{}", response);
        }
        SupplierCommands::Reclaim { device, force } => {
            let response = send_request(
                SupplierRequest::ReclaimDevice {
                    device_id: device.to_owned(),
                    force,
                },
                &client,
            );
            println!("{}", response);
        }
    }
}

