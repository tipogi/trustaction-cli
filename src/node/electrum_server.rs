use crate::utils::format::Notifier;

use bdk::electrum_client::{self, Client, ConfigBuilder, Socks5Config, ElectrumApi, Config};
use colored::Colorize;

use super::ConnectionType;

#[derive(Debug)]
pub struct ElectrumServer {}

// #TODO: Choose better naming for the struct. This is a client that opens a connection against the electrum server
impl ElectrumServer {

    pub fn connect(url: String, connection: ConnectionType) -> Client {
        println!("Trying to connect to {} electrum server...", url);
        let client = match connection {
            ConnectionType::Clearnet   => Client::new(&url),
            ConnectionType::Tor        => {
                let connection = ElectrumServer::open_tor_circuit();
                Client::from_config(&url, connection)
            }
        };
        
        match client {
            Ok(conn) => {
                Notifier::display_success(format!("{}: {}",ElectrumSuccess::ElectrumOpen.as_str(), url.italic()));
                let res = conn.server_features();
                match res {
                    Ok(success) => Notifier::display_info(
                        format!(
                            "{:#?}: {:#?}", 
                            ElectrumSuccess::ElectrumVersion, success.server_version.to_string()
                        ).to_string()),
                    Err(e)                  => ElectrumServer::error_message(e)
                }
                return conn;
            },
            Err(electrum_error)  => {
                ElectrumServer::error_message(electrum_error);
                panic!("Cannot establish connection")
            }
        }
    }

    fn open_tor_circuit() -> Config {
        let proxy = Socks5Config::new("127.0.0.1:9050");
        match ConfigBuilder::new().socks5(Some(proxy)) {
            Ok(connection) => connection.build(),
            Err(_) => panic!("ERROR: It was an error while we were creating a TOR circuit")
        }
    }

    pub fn error_message(e: electrum_client::Error) {
        if let electrum_client::Error::IOError(io_error) = e {
            // #TODO: Cannot reach to internal struct variable: repr. Also is not public ErrorData enum
            if !io_error.raw_os_error().is_none() && io_error.raw_os_error().unwrap() == 61 {
                Notifier::display_error(io_error.kind().to_string(), ElectrumError::TorProxy.as_str());
            } else {
                Notifier::display_error(io_error.kind().to_string(), io_error.to_string());
            }            
        }

        else if let bdk::electrum_client::Error::AllAttemptsErrored(errors) = e {
            for error in errors.iter() {
                if let electrum_client::Error::IOError(io_error) = error {
                    Notifier::display_error(io_error.kind().to_string(), io_error.to_string())
                } else {
                    println!("{:#?}", error);
                }
            }
        } else {
            println!("{:#?}", e);
        }
    }
}

#[derive(Debug)]
pub enum ElectrumSuccess {
    ElectrumOpen,
    ElectrumVersion,
}

impl ElectrumSuccess {
    pub fn as_str(&self) -> String {
        let message = match self {
            ElectrumSuccess::ElectrumOpen => "Opened succesfully the connection against electrum server",
            ElectrumSuccess::ElectrumVersion => "Electrum server version"
        };
        String::from(message)
    }
}

pub enum ElectrumError {
    TorProxy
}

impl ElectrumError {
    pub fn as_str(&self) -> String {
        let message = match self {
            ElectrumError::TorProxy => "TOR service might not be running or the proxy address:port is not correct one"
        };
        String::from(message)
    }
}