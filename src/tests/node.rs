#[cfg(test)]
mod tests {
    use trustaction_cli::node::electrum_server::ElectrumServer;
    use trustaction_cli::node::ConnectionType;

    #[ignore]
    #[test]
    fn connect_clearnet_electrum_server(){
        // There is some problem with the ssl certificate validation
        const HOST: &str = "testnet.aranguren.org";
        const PORT: &str = "51002";
        const PROTOCOL: &str = "ssl";
        let url = format!("{}://{}:{}", PROTOCOL, HOST, PORT);
        ElectrumServer::connect(url, ConnectionType::Clearnet);
    }

    #[ignore]
    #[test]
    fn connect_tor_electrum_server() {
        const HOST: &str = "3tc6nefii2fwoc66dqvrwcyj64dd3r35ihgxvp4u37itsopns5fjtead.onion";
        const PORT: &str = "50001";
        const PROTOCOL: &str = "tcp";
        let url = format!("{}://{}:{}", PROTOCOL, HOST, PORT);
        ElectrumServer::connect(url, ConnectionType::Tor);
    }
}   