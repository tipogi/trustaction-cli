pub mod electrum_server;

#[derive(PartialEq, Debug)]
pub enum ConnectionType {
    Clearnet,
    Tor
}

#[derive(PartialEq, Debug)]
pub enum NetworkType {
    Mainnet,
    Testnet
}