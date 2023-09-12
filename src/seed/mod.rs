use bdk::bitcoin::util::bip32::{ExtendedPrivKey, ExtendedPubKey, KeySource};
use bdk::bitcoin::hashes::hex::FromHex;
use bdk::bitcoin::Network;
use bdk::bitcoin::secp256k1::Secp256k1;
use bdk::bitcoin::util::bip32::DerivationPath;
use bdk::keys::{ExtendedKey, DerivableKey};
use bdk::keys::bip39::{Mnemonic, Language};
use bdk::miniscript::Legacy;
use bdk::keys::DescriptorKey::{self, Public, Secret};
use bdk::bitcoin::{PublicKey, PrivateKey, Address};

use std::str::FromStr;

#[derive(Debug)]
pub struct ExtendedKeys (
    pub ExtendedPrivKey, 
    pub ExtendedPubKey
);

#[derive(PartialEq, Eq, Debug)] 
pub enum KeyType {
    Private,
    Public
}

pub struct BSeed {}

impl BSeed {

    pub fn _read_mnemonic_phrase() -> String {
        String::from("warfare scale peace loud cake photo force type mask radio perfect visual")
    }
    
    // TODO: Giving a seed parse to HEX formt 
    pub fn _mnemonic_hex() -> String {
        String::from("5a2a5724f81332bcde63455eb6c15da12bc8079ff1fd5af829a999b95fc259e1845f0ccb441af8f5f7ff4d259935acea9e64dd24210136544b522dbd4ef23674")
    }
    
    pub fn _create_master_key_from_bdk_library(mnemonic: String) -> ExtendedPrivKey {
        let xkey: ExtendedKey = Mnemonic::parse_in(Language::English, mnemonic)
            .unwrap()
            .into_extended_key()
            .unwrap();
        xkey.into_xprv(Network::Testnet).unwrap()
    }
    
    pub fn _create_master_key_from_bitcoin_library(hex: String) -> ExtendedPrivKey {
        let seed_vec = Vec::from_hex(&hex).unwrap();
        ExtendedPrivKey::new_master(Network::Testnet, &seed_vec).unwrap()
    }
    
    pub fn _generate_keys(derivation_path: &str) -> ExtendedKeys {
        let mnemonic = Self::_read_mnemonic_phrase();
        let master_key = Self::_create_master_key_from_bdk_library(mnemonic);
        Self::_get_extended_keys(master_key, derivation_path)
    }
    
    pub fn _create_descriptor(extended_keys: &ExtendedKeys, derivation_path: &str, key_type: KeyType) -> String {
        let secp = Secp256k1::new();
        let deriv_path: DerivationPath = DerivationPath::from_str(derivation_path).unwrap();
        // #TODO: `Err` value: CannotDeriveFromHardenedKey'
        //let derived_xpb = &extended_keys.1.derive_pub(&secp, &deriv_path).unwrap();
    
        let origin:KeySource;
        let descriptor_key: DescriptorKey<Legacy>; 
        let mut descriptor_str: String = "".to_string();
    
        if KeyType::Private == key_type 
        {
            origin = (extended_keys.0.fingerprint(&secp), deriv_path);
            descriptor_key = extended_keys.0
                .into_descriptor_key(Some(origin), DerivationPath::default()).unwrap();
    
            if let Secret(key, _, _) = descriptor_key 
            {
                descriptor_str = key.to_string();
            }
            
        } 
        else
        {
            origin = (extended_keys.1.fingerprint(), deriv_path);
            descriptor_key = extended_keys.1
                .into_descriptor_key(Some(origin), DerivationPath::default()).unwrap();
    
            if let Public(key, _, _) = descriptor_key 
            {
                descriptor_str = key.to_string();
            }
        }
    
        let mut descriptor = "pkh(".to_string();
        descriptor.push_str(&descriptor_str.to_string());
        descriptor.push_str(")");
    
        println!("{:#?} Output Descriptor {}", key_type, descriptor);
    
        descriptor
    }
    
    pub fn _create_address_and_its_keys(address_derivation_path: &str, index: u8) {
        let secp = Secp256k1::new();
        // Generate the extended keys of the address
        let extended_keys = Self::_generate_keys(address_derivation_path);
        let ext_priv_key = extended_keys.0;
        // Secp256k1 public key used for verification of signatures
        let pub_key = ext_priv_key.private_key.public_key(&secp);
        // The first argument is the Secret 256-bit key used as x in an ECDSA signature.
        let priv_key = PrivateKey::new(ext_priv_key.private_key, Network::Testnet);
        let pub_key_2 = PublicKey::new(pub_key);
        let address = Address::p2pkh(&pub_key_2, Network::Testnet);
        println!("#{} index ({}):", index, address_derivation_path);
        println!("- Public Key: {}", pub_key);
        println!("- Private Key: {}", priv_key);
        println!("- Address: {}", address);
    }
    
    
    fn _get_extended_keys(secret_key: ExtendedPrivKey, derivation_path: &str) -> ExtendedKeys {
        let secp = Secp256k1::new();
        let derivation_path = DerivationPath::from_str(derivation_path).unwrap();
        let child_secret = secret_key.derive_priv(&secp, &derivation_path).unwrap();
        let ext_pk = ExtendedPubKey::from_priv(&secp, &child_secret);
        ExtendedKeys(child_secret, ext_pk)
    }
}

