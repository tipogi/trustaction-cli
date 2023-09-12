use bdk::miniscript::Segwitv0;
use bdk::bitcoin::util::bip32::KeySource;
use bdk::bitcoin::secp256k1::Secp256k1;
use bdk::bitcoin::util::bip32::{ ExtendedPrivKey, ExtendedPubKey};
use bdk::bitcoin::util::bip32::DerivationPath;
use bdk::keys::DescriptorKey::{self, Secret, Public};
use bdk::keys::DerivableKey;

use super::ExtendedKeys;

pub struct OutputDescriptor {}

impl OutputDescriptor {

    pub fn create_descriptor_secret_key(key: ExtendedPrivKey, derivation_path: &DerivationPath) -> String {
        let mut descriptor_str: String = "".to_string();

        let secp = Secp256k1::new();
        let origin: KeySource = (key.fingerprint(&secp), derivation_path.clone());

        // #TODO: Do no understant why adding different context (Legacy, Segwitv0) in DescriptorKey cannot see any difference
        // More info: bdk/src/descriptor/dsl.rs -> test_script_context_validation
        // That context is important when we use with the descriptor! macro. The macro create directly the requested 
        // output descriptor with the script type and we do not need to do the formatting. Check desc param
        //let (desc, _key_map, _valid_networks) = descriptor!(wpkh(descriptor_key)).unwrap();
        let descriptor_key: DescriptorKey<Segwitv0> = key
            .into_descriptor_key(Some(origin), DerivationPath::default()).unwrap();

        if let Secret(key, _, _) = descriptor_key 
        {
            descriptor_str = key.to_string();
        }

        descriptor_str
    }

    pub fn create_descriptor_pub_key(key: ExtendedPubKey, derivation_path: &DerivationPath) -> String {
        let mut descriptor_str: String = "".to_string();
        
        let origin: KeySource = (key.fingerprint(), derivation_path.clone());
        // Check in the above function the reason of context
        let descriptor_key: DescriptorKey<Segwitv0> = key
            .into_descriptor_key(Some(origin), DerivationPath::default()).unwrap();

        if let Public(key, _, _) = descriptor_key 
        {
            descriptor_str = key.to_string();
        }

        descriptor_str
    }

    pub fn get_extended_keys(master_key: ExtendedPrivKey, derivation_path: &DerivationPath) -> ExtendedKeys {
        let secp = Secp256k1::new();
        let child_secret = master_key.derive_priv(&secp, &derivation_path).unwrap();
        let ext_pk = ExtendedPubKey::from_priv(&secp, &child_secret);
        ExtendedKeys(child_secret, ext_pk)
    }
}