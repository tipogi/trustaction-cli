pub mod address;
pub mod output_descriptor;

use bdk::{keys::{bip39::{Mnemonic, Language}, DerivableKey, ExtendedKey}, blockchain::ElectrumBlockchain, SyncOptions, Wallet, database::MemoryDatabase, electrum_client::Client};
use bdk::bitcoin::Network;
use bdk::bitcoin::util::bip32::{ ExtendedPrivKey, ExtendedPubKey};
use bdk::bitcoin::util::bip32::DerivationPath;

use std::str::FromStr;

use self::address::ScriptType;
use self::output_descriptor::OutputDescriptor;

#[derive(Debug)]
pub struct ExtendedKeys (
    pub ExtendedPrivKey, 
    pub ExtendedPubKey
);

pub struct TrustactionWallet {
    mnemonic: Mnemonic,
    network: Network,
    script: ScriptType,
    account: u64
}

impl TrustactionWallet {
    pub fn new(mnemonic: String, network: Network, script: ScriptType, account: u64) -> Self {
        Self {
            mnemonic: Mnemonic::parse_in(Language::English, mnemonic).unwrap(),
            network,
            script,
            account
        }
    }

    pub fn create_master_key_from_bdk_library(&self) -> ExtendedPrivKey {
        let xkey: ExtendedKey = self.mnemonic.clone()
            .into_extended_key()
            .unwrap();
        xkey.into_xprv(self.network).unwrap()
    }

    pub fn create_output_descriptor(&self, secret_key: bool, external: bool) -> String {
        let master_key = self.create_master_key_from_bdk_library();
        let raw_derivation_path = self.create_derivation_path(external);
        let derivation_path: DerivationPath = DerivationPath::from_str(&raw_derivation_path).unwrap();
        let extended_keys: ExtendedKeys = OutputDescriptor::get_extended_keys(master_key, &derivation_path);

        // Start formatting the creation of the output descriptor
        let mut descriptor = self.script.into_descriptor();

        let output_descriptor = match secret_key {
            true => OutputDescriptor::create_descriptor_secret_key(extended_keys.0, &derivation_path),
            false => OutputDescriptor::create_descriptor_pub_key(extended_keys.1, &derivation_path)
        };

        descriptor.push_str(&output_descriptor);
        descriptor.push_str(")");

        descriptor
    }

    fn create_derivation_path(&self, external: bool) -> String {
        let change_index = if external {"0"} else {"1"};
        format!("m/{}/1h/{}h/{}", self.script.into_purpose(), self.account, change_index)
        //format!("m/{}/1h/{}h", self.script.into_purpose(), self.account)
    }

    pub fn sync_wallet(wallet: &Wallet<MemoryDatabase>, electrum_client: Client) {
        let blockchain = ElectrumBlockchain::from(electrum_client);
        wallet.sync(&blockchain, SyncOptions::default()).unwrap();

        println!("Descriptor balance: {} SAT", wallet.get_balance().unwrap());
    }
}