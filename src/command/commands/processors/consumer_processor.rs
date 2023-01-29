use crate::command::commands::processors::processable::Processable;
use crate::command::network::functions::send_request;
use crate::market::request::{ConsumerRequest, ConsumerResponse, Response, SysStateRequest};
use crate::market::{DeviceFilter, DeviceFilterVec};
use crate::net::TCPClient;
use crate::scrcpy::adb_utils;
use crate::scrcpy::adb_utils::SCRCPY_SHORTCUT_HELP;
use std::collections::HashSet;
use crate::command::commands::subcommands::consumer::ConsumerCommands;

pub struct ConsumerProcessor {}

impl Processable<ConsumerCommands> for ConsumerProcessor  {
    fn process(command: &ConsumerCommands, client: TCPClient) -> () {
        let _command = command.to_owned();

        match _command {
            ConsumerCommands::Status => {
                let response = send_request(ConsumerRequest::Status, &client);
                println!("{}", response);
            }
            ConsumerCommands::Start { remote, port, user } => {
                let response = send_request(
                    SysStateRequest::StartConsumer {
                        mm_host: remote,
                        mm_port: port,
                        name: user,
                    },
                    &client,
                );
                println!("{}", response);
            }
            ConsumerCommands::Stop => {
                let response = send_request(SysStateRequest::StopConsumer, &client);
                println!("{}", response);
            }
            ConsumerCommands::Reserve { device, no_default } => {
                let response = send_request(
                    ConsumerRequest::ReserveDevice {
                        device_id: device,
                        no_use: no_default,
                    },
                    &client,
                );
                println!("{}", response);
            }
            ConsumerCommands::Release { device } => {
                let response = if device.is_some() {
                    send_request(
                        ConsumerRequest::ReleaseDevice {
                            device_id: device.as_ref().unwrap().to_string(),
                        },
                        &client,
                    )
                } else {
                    send_request(ConsumerRequest::ReleaseAllDevices, &client)
                };
                println!("{}", response);
            }
            ConsumerCommands::ListAvailable => {
                let response = send_request(ConsumerRequest::GetAvailableDevices, &client);
                println!("{}", response);
            }
            ConsumerCommands::GetDevices {
                is_available,
                device_ids,
                device_names,
                device_models,
                supplied_by,
                reserved_by,
            } => {
                let mut filters = Vec::new();

                if let Some(is_available) = is_available {
                    filters.push(DeviceFilter::IsAvailable(is_available));
                }

                if let Some(device_ids) = device_ids {
                    let device_ids = HashSet::from_iter(device_ids);
                    filters.push(DeviceFilter::DeviceIds(device_ids));
                }
                if let Some(device_names) = device_names {
                    let device_names = HashSet::from_iter(device_names);
                    filters.push(DeviceFilter::DeviceNames(device_names));
                }
                if let Some(device_models) = device_models {
                    let device_models = HashSet::from_iter(device_models);
                    filters.push(DeviceFilter::DeviceModels(device_models));
                }
                if let Some(supplied_by) = supplied_by {
                    let supplied_by = HashSet::from_iter(supplied_by);
                    filters.push(DeviceFilter::SupplierNames(supplied_by));
                }
                if let Some(reserved_by) = reserved_by {
                    let reserved_by = HashSet::from_iter(reserved_by);
                    filters.push(DeviceFilter::ConsumerNames(reserved_by));
                }
                let filter_vec = DeviceFilterVec { filters };
                let response =
                    send_request(ConsumerRequest::GetDevicesByFilter { filter_vec }, &client);
                println!("{}", response);
            }
            ConsumerCommands::ListReserved => {
                let response = send_request(ConsumerRequest::Status, &client);
                match response {
                    Response::Consumer(ConsumerResponse::Status { state }) => {
                        let reserved_devices = state.devices;
                        let using_device = state.using_device.unwrap_or_default();
                        println!("Reserved devices:");
                        for (_, device) in reserved_devices.iter() {
                            println!("{}", device);
                            if device.device_id == using_device {
                                println!(
                                    r"        |
|----> Current default device"
                                );
                            } else {
                                println!(
                                    r"        |
|----> Available on port: {}",
                                    device.used_by_port
                                );
                            }
                        }
                        println!();
                    }
                    _ => println!("Unexpected response: {}", response),
                }
            }
            ConsumerCommands::SetDefault { device } => {
                let response =
                    send_request(ConsumerRequest::UseDevice { device_id: device }, &client);
                println!("{}", response);
            }
            ConsumerCommands::Scrcpy { device, args } => {
                let scrcpy_args = adb_utils::get_scrcpy_args(args);
                let response = send_request(
                    ConsumerRequest::StartScrCpy {
                        device_id: device,
                        scrcpy_args,
                    },
                    &client,
                );
                println!("{}", response);
            }
            ConsumerCommands::StopScrcpy { device } => {
                let response =
                    send_request(ConsumerRequest::StopScrCpy { device_id: device }, &client);
                println!("{}", response);
            }
            ConsumerCommands::SetScrcpyArgs(args) => {
                let scrcpy_args = adb_utils::get_scrcpy_args(args);
                let response =
                    send_request(ConsumerRequest::SetScrCpyDefaults { scrcpy_args }, &client);
                println!("{}", response);
            }
            ConsumerCommands::GetScrcpyArgs => {
                let response = send_request(ConsumerRequest::GetScrCpyDefaults, &client);
                println!("{}", response);
            }
            ConsumerCommands::ScrcpyShortcuts => {
                println!("{}", SCRCPY_SHORTCUT_HELP);
            }
        }
    }
}
