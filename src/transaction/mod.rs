pub mod legacy;
pub mod format;

use std::ops::Deref;

use crate::transaction::legacy::TxParser;
use crate::utils::reader::{ TrustactionEnv, EnvParams };
use crate::node::electrum_server::ElectrumServer;
use crate::node::ConnectionType;
use crate::wallet::TrustactionWallet;
use crate::wallet::address::ScriptType;

use bdk::bitcoin::consensus::encode::serialize_hex;
use bdk::wallet::AddressIndex;
use bdk::{Wallet, SignOptions};
use bdk::bitcoin::Network::{Testnet, self};
use bdk::database::{MemoryDatabase, Database};


pub struct BTransaction {}

impl BTransaction {
    pub fn serialize_transaction() -> Result<(), Box<dyn std::error::Error>>{
        let params = TrustactionEnv::initialise().unwrap();
        let electrum_tor_host = params.get(EnvParams::TorElectrumServer);
        // #TODO: Before connect make generic the connection type
        let electrum_client = ElectrumServer::connect(electrum_tor_host, ConnectionType::Tor);
        //
        let wallet_init = TrustactionWallet::new(
            params.get(EnvParams::Seed),
            Testnet,
            //ScriptType::NativeSegwit,
            ScriptType::Legacy,
            0
        );
        // The ones that we use to create a public addresses
        let external_descriptor = wallet_init.create_output_descriptor(true, true);
        // The change addresses
        let internal_descriptor = wallet_init.create_output_descriptor(true, false);

        let wallet: Wallet<MemoryDatabase> = Wallet::new(
            &external_descriptor,
            Some(&internal_descriptor),
            Network::Testnet,
            MemoryDatabase::default(),
        )?;

        // If we do not sync the wallet, the wallet balance is going to be zero. Before start, sync the wallet
        TrustactionWallet::sync_wallet(&wallet, electrum_client);
        // That one should be generic
        let receive_address = wallet.get_address(AddressIndex::New)?;
        println!("Generated Address: {}", receive_address);
        
        Self::get_wallet_utxos(&wallet);

        let mut tx_builder = wallet.build_tx();

        // TODO: Error when we do not have money
        tx_builder
            .add_recipient(receive_address.script_pubkey(), 1000)

            .enable_rbf();
        let (mut psbt, _) = tx_builder.finish().unwrap();

        println!("Transaction details: {:#?}", psbt);

        let finalized = wallet.sign(&mut psbt, SignOptions::default()).unwrap();
        assert!(finalized, "Tx has not been finalized");
        println!("Transaction Signed: {}", finalized);
        println!("Transaction details: {:#?}", psbt);
        
        let raw_transaction = psbt.extract_tx();
        let serialized_tx = serialize_hex(&raw_transaction);
        TxParser::from_hex(serialized_tx);

        

        Ok(())

    }

    fn get_wallet_utxos(wallet: &Wallet<MemoryDatabase>) {
        let ddbb = wallet.database();
        let utxos = ddbb.deref().iter_utxos();

        println!("{:#?}", utxos);
    }


}