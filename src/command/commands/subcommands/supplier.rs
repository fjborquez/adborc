use crate::util::SysStateDefaultConfig;
use clap::Subcommand;

#[derive(Subcommand, Clone)]
pub enum SupplierCommands {
    /// Get the current status of Supplier.
    Status,
    /// Connect to a network (MarketMaker) and start Supplier mode on the system.
    Start {
        /// Hostname or IP address of the MarketMaker.
        #[clap(value_parser)]
        remote: String,
        /// Port of the MarketMaker.
        #[clap(short, long, value_parser, default_value_t = SysStateDefaultConfig::BIND_PORT)]
        port: u16,
        /// Optional name to be specified for the Supplier.
        /// This name will be used to identify the Supplier in the network.
        /// Defaults to hostname of the Supplier machine.
        /// Note: This name is for reference only and does not affect the
        /// functionality of the Supplier. There is no need to specify a unique name.
        #[clap(short, long, value_parser)]
        user: Option<String>,
        /// Use encrypted communication channels for device. This is recommended.
        /// If not specified, all device communication will be unencrypted.
        /// If specified, encrypted tunnels will be created for device communication.
        #[clap(short, long, action)]
        secure: bool,
    },
    /// Terminate Supplier mode on the system. Supplier will be removed from the
    /// network and all supplied devices will be reclaimed.
    Stop,
    /// Supply devices to the network.
    Supply {
        /// List of devices, specified by their `ANDROID_SERIAL`, to supply.
        /// If no devices are specified, all devices detected on `ADB` will be supplied.
        /// Devices must be separated by a comma.
        /// Example: adborc supply --devices "serial1,serial2,serial3"
        #[clap(long, value_parser, use_value_delimiter = true)]
        devices: Option<Vec<String>>,
    },
    /// Reclaim a device from the network.
    /// If the device is currently in use, reclaim will fail.
    /// Use option `-f/--force` to force the reclaim.
    Reclaim {
        /// Device to reclaim, specified by its device_id.
        #[clap(value_parser)]
        device: String,
        /// Optional flag to force the reclaim.
        #[clap(short, long, action)]
        force: bool,
    },
}
