use crate::command::commands::{
    consumer::subcommand::ConsumerCommands, marketmaker::subcommand::MarketMakerCommands,
    supplier::subcommand::SupplierCommands,
};
use clap::Subcommand;

#[derive(Subcommand, Clone)]
pub enum Commands {
    /// Starts the system listener
    Init,
    /// Get the current status of the system. Returns if the system
    /// is initialized, and if it is, the mode(s) currently active.
    Status,
    /// Shutdown the system. Terminates all active modes (MarketMaker/Supplier/Consumer).
    Shutdown,
    /// Get the network_id of system.
    GetNetworkId,
    /// Check if `adb` and `scrcpy` are installed and compatible.
    /// Outputs which modes (MarketMaker/Supplier/Consumer) are
    /// available to be run on the system.
    Check,
    /// Set the path to the `adb` executable, if not in PATH.
    /// The specified path must be an absolute path to the `adb` executable.
    /// For example: `C:\Users\user\Downloads\platform-tools_r30.0.4-windows\adb.exe`
    SetAdbPath {
        #[clap(value_parser)]
        path: String,
    },
    /// Set the path to the `scrcpy` executable, if not in PATH.
    /// The specified path must be an absolute path to the `scrcpy` executable.
    /// For example: `C:\Users\user\Downloads\scrcpy-win64-v1.17\scrcpy.exe`
    SetScrcpyPath {
        #[clap(value_parser)]
        path: String,
    },
    #[cfg(feature = "mangen")]
    /// Generate manual page for `adborc`.
    Mangen {
        /// Optional path to place generated man page in.
        /// Path must be an existing directory. Man page will be placed
        /// in that directory with the name `adborc.man`.
        /// If path is not specified, man page will be placed
        /// in the current executable's directory with the name `adborc.man`.
        #[clap(short, long, value_parser)]
        path: Option<String>,
    },

    /// MarketMaker subcommand. Commands to MarketMaker, if the system is running in
    /// MarketMaker mode. Use `adborc marketmaker help` for more information.
    #[clap(subcommand)]
    Marketmaker(MarketMakerCommands),
    /// Supplier subcommand. Commands to Supplier, if the system is running in
    /// Supplier mode. Use `adborc supplier help` for more information.
    #[clap(subcommand)]
    Supplier(SupplierCommands),
    /// Consumer subcommand. Commands to Consumer, if the system is running in
    /// Consumer mode. Use `adborc consumer help` for more information.
    #[clap(subcommand)]
    Consumer(ConsumerCommands),
}
