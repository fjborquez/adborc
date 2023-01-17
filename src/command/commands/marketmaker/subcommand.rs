use clap::Subcommand;

#[derive(Subcommand, Clone)]
pub enum MarketMakerCommands {
    /// Get the current status of MarketMaker.
    Status,
    /// Start a network by running MarketMaker mode on the system.
    Start,
    /// Terminate the MarketMaker on the system.
    /// WARNING: This will terminate the entire network of Suppliers and Consumers
    /// connected to the MarketMaker.
    Stop,
    /// Enable MarketMaker whitelisting for authenticating
    /// Suppliers and Consumers. When whitelisting is enabled,
    /// only Suppliers and Consumers whose `network_id` (Use `adborc get-network-id`)
    /// is added to the whitelist will be able to connect to the network.
    /// Use `adborc marketmaker add-supplier` and `adborc marketmaker add-consumer`
    /// to add Suppliers and Consumers to the whitelist.
    /// Whitelist is disabled by default.
    UseWhitelist,
    /// Remove the whitelisting requirement for Suppliers and Consumers.
    ResetWhitelist,
    /// Add a supplier to the whitelist.
    AddSupplier {
        /// The network_id of Supplier.
        peer_id: String,
    },
    /// Remove a supplier from the whitelist.
    /// Note: This will not terminate the Supplier from the network if it is already
    /// connected to the MarketMaker.
    RemoveSupplier {
        /// The `network_id` of Supplier.
        peer_id: String,
    },
    /// Add a consumer to the whitelist.
    AddConsumer {
        /// The `network_id` of Consumer.
        peer_id: String,
    },
    /// Remove a consumer from the whitelist.
    /// Note: This will not terminate the Consumer from the network if it is already
    /// connected to the MarketMaker.
    RemoveConsumer {
        /// The `network_id` of Consumer.
        peer_id: String,
    },
}
