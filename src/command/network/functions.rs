use std::str::FromStr;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};
use std::io;
use std::io::Error;
use log::error;
use crate::market::request::{Response, SysStateRequest, SysStateResponse, ToJson};
use crate::market::SysState;
use crate::net::TCPClient;
use crate::scrcpy::{ADBORC_VERSION, SysStateDefaultConfig};

pub fn send_request<T>(request: T, client: &TCPClient) -> Response
where
    T: ToJson,
{
    let response = client.send_request(request, None).unwrap_or_else(|e| {
        SysStateResponse::RequestProcessingError {
            reason: e.to_string(),
        }
        .to_json()
    });
    Response::from_str(&response).unwrap()
}

pub fn check_listener() -> bool {
    let addr = SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        SysStateDefaultConfig::BIND_PORT,
    );
    let stream = TcpStream::connect(addr);
    stream.is_ok()
}

pub fn init_listener() -> io::Result<()> {
    println!("Starting system listener...");
    SysState::start_system()
}

pub fn print_response<T>(request: T, client: &TCPClient)
where
    T: ToJson,
{
    let response = send_request(request, &client);
    println!("{}", response);
}

pub fn init_system_listener() {
    let init_result = init_listener();

    match init_result {
        Ok(_) => {},
        Err(result) => init_system_listener_error(result)
    }
}

pub fn init_system_listener_error(error: Error) {
    error!("Failed to start system listener");
    println!(
        "Failed to start system listener\n{}",
        error
    );
}

pub fn check_client_compatibility(client: &TCPClient) {
    // Check if the AdbOrc client is compatible with the listener, before proceeding.
    // Note: Pre-0.2.0 versions, this check is not performed. This may lead to unhelpful error
    // message, if the client and server api are not compatible.
    let response = send_request(
        SysStateRequest::CheckVersion {
            version: ADBORC_VERSION.to_string(),
        },
        &client,
    );
    match response {
        Response::System(SysStateResponse::ClientOk) => {} // Do nothing.
        Response::System(SysStateResponse::ClientError { reason }) => println!("{}", reason),
        _ => println!("Client version {} is not supported by listener version: 0.1.0", ADBORC_VERSION),
    }
}
