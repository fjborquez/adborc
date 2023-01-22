use crate::util::adb_utils::ScrcpyCliArgs;
use crate::util::SysStateDefaultConfig;
use clap::Subcommand;

#[derive(Subcommand, Clone)]
pub enum ConsumerCommands {
    /// Get the current status of Consumer.
    Status,
    /// Connect to a network (MarketMaker) and start Consumer mode on the system.
    Start {
        /// Hostname or IP address of the MarketMaker.
        #[clap(value_parser)]
        remote: String,
        /// Port of the MarketMaker.
        #[clap(short, long, value_parser, default_value_t = SysStateDefaultConfig::BIND_PORT)]
        port: u16,
        /// Optional name to be specified for the Consumer.
        /// This name will be used to identify the Consumer in the network.
        /// Defaults to hostname of the Consumer machine.
        /// Note: This name is for reference only and does not affect the
        /// functionality of the Consumer. There is no need to specify a unique name.
        #[clap(short, long, value_parser)]
        user: Option<String>,
    },
    /// Terminate Consumer mode on the system. Consumer will be removed from the
    /// network and all reserved devices will be added back to the network.
    Stop,
    /// Request to reserve a device from the MarketMaker. If the device is available,
    /// it will be reserved for the Consumer and tunnels (encrypted, if the device
    /// Supplier uses secure mode) will be setup for device communication. The device
    /// will be available for use on the Consumer system using `adb` on the specified port.
    /// If the device is not available, the request will fail.
    Reserve {
        /// `device_id` of the device to reserve.
        /// Use `adborc consumer list-available` for a list of available devices.
        #[clap(value_parser)]
        device: String,
        /// Optional flag to not make the device default.
        /// A default device is available on the system using `adb` without specifying
        /// the port number for the device.
        /// If not specified and default device not already set, the device
        /// will be set as the default device.
        #[clap(long, action)]
        no_default: bool,
    },
    /// Release a device. If not specified, all reserved devices will be released.
    Release {
        /// `device_id` of the device to be released.
        /// Use `adborc consumer list-reserved` for a list of reserved devices.
        #[clap(value_parser)]
        device: Option<String>,
    },
    /// Get a list of all available devices on the network.
    ListAvailable,
    /// Get devices in the network and filter them by some criteria.
    GetDevices {
        /// If `is_available` is true, only available devices will be returned.
        /// If `is_available` is false, only reserved devices will be returned.
        /// If `is_available` is not specified, all devices will be returned.
        #[clap(long, value_parser)]
        is_available: Option<bool>,
        /// List of device_ids to filter devices by.
        /// If specified, only devices with device_ids in the list will be returned.
        /// Devices must be separated by a comma.
        /// Example: adborc consumer get-devices --device_ids "id1,id2,id3"
        #[clap(long, value_parser, use_value_delimiter = true)]
        device_ids: Option<Vec<String>>,
        /// List of device names to filter devices by.
        /// If specified, only devices with names in the list will be returned.
        /// Names must be separated by a comma.
        /// Example: adborc consumer get-devices --device_names "name1,name2,name3"
        #[clap(long, value_parser, use_value_delimiter = true)]
        device_names: Option<Vec<String>>,
        /// List of device models to filter devices by.
        /// If specified, only devices with models in the list will be returned.
        /// Models must be separated by a comma.
        /// Example: adborc consumer get-devices --device_models "model1,model2,model3"
        #[clap(long, value_parser, use_value_delimiter = true)]
        device_models: Option<Vec<String>>,
        /// List of device supplier names to filter devices by.
        /// If specified, only devices supplied by the specified supplier(s) in the list will be returned.
        /// Suppliers must be separated by a comma.
        /// Example: adborc consumer get-devices --device_suppliers "supplier1,supplier2,supplier3"
        #[clap(long, value_parser, use_value_delimiter = true)]
        supplied_by: Option<Vec<String>>,
        /// List of device consumer names to filter devices by.
        /// If specified, only devices reserved by the specified consumer(s) in the list will be returned.
        #[clap(long, value_parser, use_value_delimiter = true)]
        reserved_by: Option<Vec<String>>,
    },
    /// Show currently reserved devices.
    ListReserved,
    /// Set a device as the default device.
    SetDefault {
        /// `device_id` of the device.
        #[clap(value_parser)]
        device: String,
    },
    /// Start device screen mirroring using `scrcpy` for a device.
    /// Checkout: `<https://github.com/Genymobile/scrcpy>` for more information on `scrcpy`.
    Scrcpy {
        /// `device_id` of the device to start scrcpy for.
        #[clap(value_parser)]
        device: String,
        #[clap(flatten)]
        args: ScrcpyCliArgs,
    },
    /// Stop device screen mirroring for a device.
    /// Note: This doesn't work if the Consumer and Supplier are on the same machine.
    ///       see: https://github.com/mobi-nex/adborc/issues/16 for more information.
    StopScrcpy {
        /// `device_id` of the device to stop scrcpy for.
        #[clap(value_parser)]
        device: String,
    },
    /// Set the default arguments for `scrcpy`.
    SetScrcpyArgs(ScrcpyCliArgs),
    /// Get the default arguments for `scrcpy` if set using `adborc consumer set-scrcpy-args`.
    GetScrcpyArgs,
    /// Show scrcpy shortcuts.
    ScrcpyShortcuts,
}
