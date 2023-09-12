use bdk::bitcoin::Network;
use dotenv::dotenv;
use envy::Error;
use serde::Deserialize;

use super::format::Notifier;

// Parameters of the environment file
pub enum EnvParams {
    Network,
    TorElectrumServer,
    ClearnetElectrumServer,
    Seed,
    Xpub,
    Descriptor
}

#[derive(Deserialize, Debug)]
pub struct TrustactionEnv {
    pub network: Network,
    tor_electrum_server: String,
    electrum_host: String,
    seed: String,
    xpub: String,
    descriptor: String
}

impl TrustactionEnv {

    pub fn initialise() -> Result<Self, String> {
        // Load the environment variables from the ".env" file.
        dotenv().ok();
        
        match envy::from_env::<TrustactionEnv>() {
            Ok(env) => Ok(env),
            Err(e) => {
                let mut message: String = String::from("the .env file does not exist");
                println!("{:#?}", e);
                if let Error::MissingValue(envy_message) = e {
                    message = format!("Missing param: {:#}", envy_message);
                } else if let Error::Custom(custom_message) = e {
                    message = format!("{:#}", custom_message);
                }
                Notifier::display_error("Envy error".to_string(), message.clone());
                panic!("{}", message)
            }
        }
    }

    pub fn get(&self, name: EnvParams) -> String {
        let param = match name {
            EnvParams::Network => return self.network.to_string(),
            EnvParams::TorElectrumServer => &self.tor_electrum_server,
            EnvParams::ClearnetElectrumServer => &self.electrum_host,
            EnvParams::Seed => &self.seed,
            EnvParams::Xpub => &self.xpub,
            EnvParams::Descriptor => &self.descriptor
        };
        param.clone()
    }



    // DEPRECATED: Now we use dotenv and envy together to get struct of the environment
    /*fn _get_seed() -> Result<String, String> {
        // Load the environment variables from the ".env" file.
        dotenv().ok();

        match env::var(EnvParams::Seed.to_string()) {
            Ok(seed) => Ok(seed),
            Err(e) => Err(String::from("We could not find the seed to create a wallet"))
        }
    }*/
}