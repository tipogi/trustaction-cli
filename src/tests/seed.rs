#[cfg(test)]
mod tests {
    use txbox::seed::*;
    
    #[test]
    fn test_words_master_key() {
        let mnemonic = BSeed::_read_mnemonic_phrase();
        let master_key = BSeed::_create_master_key_from_bdk_library(mnemonic);

        assert_eq!(master_key.to_string(), "tprv8ZgxMBicQKsPfN4jv4fCUSYqhu5PhMXDx6XXLiRspg8SrTFefMgyLyXy4cxkjeTwaSTqEvr1VCRnP8SdQPGDMiVueExRXLyLvESumxbyq9v");
    }

    #[test]
    fn test_hex_master_key() {
        let hex_mnemonic = BSeed::_mnemonic_hex();
        let master_key = BSeed::_create_master_key_from_bitcoin_library(hex_mnemonic);

        assert_eq!(master_key.to_string(), "tprv8ZgxMBicQKsPfN4jv4fCUSYqhu5PhMXDx6XXLiRspg8SrTFefMgyLyXy4cxkjeTwaSTqEvr1VCRnP8SdQPGDMiVueExRXLyLvESumxbyq9v");
    }

    #[test]
    fn test_extended_keys() {
        // DerivationPath: m/purpose'/coin_type'/account'/change/address_index
        // Account extended: "m/44h/1h/0h"
        // BIP32 extended Key: "m/44h/1h/0h/0"
        const DERIVATION_PATH: &str = "m/44h/1h/0h";
        
        let extended_keys = BSeed::_generate_keys(DERIVATION_PATH);

        assert_eq!(extended_keys.0.to_string(), "tprv8fmAG7oTXacxLHjhGTTBx8Nmi5tBBv2ALJ56qC2PsBuQeydGRRUT7mGsNmqoXWvtbZxnP2dnc5KmXJUYKny6HgyMPBaT2G1axefxRHrFffX");
        assert_eq!(extended_keys.1.to_string(), "tpubDCTCQXqhfxJdDkmVA77nMY2tH7Q7MFD4ubft7i4hHThoVTt33pJ3JFtjYuVXEspHx6UaTeqsKykJnV2cwr4PDDzUVdmHDcELgq2FYCyHXXy");
    }

    #[test]
    fn test_extended_key_descriptor() {
        // Account extended
        const DERIVATION_PATH: &str = "m/44h/1h/0h";

        let extended_keys = BSeed::_generate_keys(DERIVATION_PATH);

        let private_descriptor = BSeed::_create_descriptor(&extended_keys, DERIVATION_PATH, KeyType::Private);
        let public_descriptor = BSeed::_create_descriptor(&extended_keys, DERIVATION_PATH, KeyType::Public);

        assert_eq!(private_descriptor, "pkh([20ceec93/44'/1'/0']tprv8fmAG7oTXacxLHjhGTTBx8Nmi5tBBv2ALJ56qC2PsBuQeydGRRUT7mGsNmqoXWvtbZxnP2dnc5KmXJUYKny6HgyMPBaT2G1axefxRHrFffX/*)");
        assert_eq!(public_descriptor, "pkh([20ceec93/44'/1'/0']tpubDCTCQXqhfxJdDkmVA77nMY2tH7Q7MFD4ubft7i4hHThoVTt33pJ3JFtjYuVXEspHx6UaTeqsKykJnV2cwr4PDDzUVdmHDcELgq2FYCyHXXy/*)");

    }

    #[test]
    fn create_multiple_addresses() {
        // Account/Receive
        const DERIVATION_PATH: &str = "m/44h/1h/0h/0/";
        // Account extended
        for index in 0..=5 {
            let index_str: String = index.to_string();
            let mut address_derivation_path = String::from(DERIVATION_PATH);
            address_derivation_path.push_str(&index_str);
            BSeed::_create_address_and_its_keys(&address_derivation_path, index);
        }
    }
}