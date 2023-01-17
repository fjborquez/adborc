use crate::command::commands::commands::Commands;
use crate::command::commands::consumer::command_processor::process_consumer_command;
use crate::command::commands::marketmaker::command_processor::process_marketmaker_command;
use crate::command::commands::supplier::command_processor::process_supplier_command;
use crate::command::daemon::functions::init_background_task;
use crate::command::network::functions;
use crate::command::network::functions::{
    check_client_compatibility, check_listener, init_system_listener,
};
use crate::market::request::SysStateRequest;
use crate::net::TCPClient;
use crate::util::SysStateDefaultConfig;

pub fn process_command(command: Commands, client: TCPClient) {
    match command {
        Commands::Status => functions::print_response(SysStateRequest::GetState, &client),
        Commands::Shutdown => functions::print_response(SysStateRequest::Shutdown, &client),
        Commands::GetNetworkId => functions::print_response(SysStateRequest::GetPeerId, &client),
        Commands::Check => functions::print_response(SysStateRequest::SystemCheck, &client),
        Commands::SetAdbPath { path } => functions::print_response(
            SysStateRequest::SetAdbPath {
                adb_path: path.to_owned(),
            },
            &client,
        ),
        Commands::SetScrcpyPath { path } => functions::print_response(
            SysStateRequest::SetScrcpyPath {
                scrcpy_path: path.to_owned(),
            },
            &client,
        ),
        Commands::Marketmaker(cmd)  => process_marketmaker_command(cmd, client),
        Commands::Supplier(cmd) => process_supplier_command( cmd, client),
        Commands::Consumer(cmd) => process_consumer_command( cmd, client),
        _ => {
            println!("Not yet implemented");
        }
    }
}

pub fn init() {
    if check_listener() {
        println!(
            "Either the default port ({}) is occupied or system listener is already running",
            SysStateDefaultConfig::BIND_PORT
        );
        return;
    } else {
        init_background_task();
        init_system_listener();
    }
}

pub fn execute_command(command: Commands) {
    let client = TCPClient::new("127.0.0.1", SysStateDefaultConfig::BIND_PORT).unwrap();
    check_client_compatibility(&client);
    process_command(command, client);
}

pub fn setup_mangen(_command: Commands) {
    #[cfg(feature = "mangen")]
    {
        use crate::command::mangen::functions::create_mangen;
        // Process 'mangen' command separately.

        if let Commands::Mangen { path } = _command.to_owned() {
            create_mangen(path);
        }
    }
}

pub fn setup_listener(command: Commands) {
    // Process 'init' command separately.
    if let Commands::Init = command {
        init();
    } else if !check_listener() {
        println!(
            "System listener is not running. Use \"adborc init\" to start the system listener."
        );
    }
}
