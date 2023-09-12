use bdk::bitcoin::Txid;
use bdk::bitcoin::consensus::encode::serialize_hex;
use bdk::bitcoin::hashes::hex::ToHex;
use bdk::{FeeRate, Wallet, SyncOptions, SignOptions};
use bdk::database::MemoryDatabase;
use bdk::blockchain::ElectrumBlockchain;
use bdk::electrum_client::Client;
//use bitcoin::consensus::serialize;
use bdk::wallet::AddressIndex::New;

use std::str::FromStr;
use bdk::bitcoin::util::psbt::PartiallySignedTransaction as Psbt;

fn main() -> Result<(), bdk::Error> {
    let client = Client::new("ssl://electrum.blockstream.info:60002")?;
    let wallet = Wallet::new(
        "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)",
        Some("wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/1/*)"),
        bdk::bitcoin::Network::Testnet,
        MemoryDatabase::default(),
    )?;
    let blockchain = ElectrumBlockchain::from(client);

    wallet.sync(&blockchain, SyncOptions::default())?;

    let send_to = wallet.get_address(New)?;
    let (mut psbt, details) = {
        let mut builder =  wallet.build_tx();
        builder
            .add_recipient(send_to.script_pubkey(), 30_000)
            .enable_rbf()
            .do_not_spend_change()
            .fee_rate(FeeRate::from_sat_per_vb(2.0));
        builder.finish()?
    };

    println!("Transaction details: {:#?}", details);
    //println!("Unsigned PSBT: {}", &psbt);

    //Ok(());



    let wallet = Wallet::new(
        "wpkh([c258d2e4/84h/1h/0h]tprv8griRPhA7342zfRyB6CqeKF8CJDXYu5pgnj1cjL1u2ngKcJha5jjTRimG82ABzJQ4MQe71CV54xfn25BbhCNfEGGJZnxvCDQCd6JkbvxW6h/0/*)",
        Some("wpkh([c258d2e4/84h/1h/0h]tprv8griRPhA7342zfRyB6CqeKF8CJDXYu5pgnj1cjL1u2ngKcJha5jjTRimG82ABzJQ4MQe71CV54xfn25BbhCNfEGGJZnxvCDQCd6JkbvxW6h/1/*)"),
        bdk::bitcoin::Network::Testnet,
        MemoryDatabase::default(),
    )?;

    //let psbt = "...";
    //let mut psbt = Psbt::from_str(psbt)?;

    let tx = psbt.extract_tx();
    let serialized_tx = serialize_hex(&tx);

    // Get the hex representation of the transaction
    println!("{:#?}", serialized_tx);

    //let finalized = wallet.sign(&mut psbt, SignOptions::default())?;

    //println!("Finalized: {:#?}", finalized);

    

    let hex = String::from("020000000151eca8ddb11f722e22a553b92116d6a4b3a72e162669031174c3f8d8c57a3653020000006b48304502210093f54fffa356435cf73bc24b873c398010177eb8b5b083a191ae230d2ac714a5022049b1b976cb6181f4cddf235d4fbe6934a7df8f5098cdbdbbc7a40562a22cff0f012102204b0e8ab26d85bdb0e39356f114ab99a4daabc635df9f9046731656e3db77d4ffffffff02e8030000000000001976a914f46b17d29ddeaa5ea5a0e00617992084b359184e88ac10e63000000000001976a914254c96a1a272651eaa81972bb67073e75571aa3988ac00000000");
    let tx_id = Txid::from_str(&hex)?;
    println!("TxId: {:#?}", tx_id);
    Ok(())
}