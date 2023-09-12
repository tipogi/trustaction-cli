use super::format::{ DARK_ORANGE, BITCOIN_COLOR, TxFormat };

use std::{error::Error, io::Read};
use buffer::ReadBuffer;
use hex::FromHex;
use colored::{ Color, Colorize };
use serde::{Serialize, Deserialize};
use sha2::{Digest, Sha256};

type TxParserResult<T> = Result<T, Box<dyn Error>>;


#[derive(Serialize, Deserialize, Debug)]
struct Input {
    #[serde(serialize_with = "TxFormat::hex_encoding")]
    txid: [u8; 32],
    vout: u32,
    script_sig: String,
    #[serde(serialize_with = "TxFormat::hex_formatting")]
    sequence: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Output {
    amount: u64,
    script_pub_key: String
}

#[derive(Serialize, Deserialize, Debug)]
struct ParsedTransaction {
    version: u32,
    inputs: Vec<Input>,
    outputs: Vec<Output>,
    locktime: u32,
    #[serde(serialize_with = "TxFormat::hex_encoding")]
    transaction_id: [u8; 32],
}



pub struct TxParser {}

impl TxParser {
    pub fn from_hex(serialized_tx: String) {
        let bytes = Vec::<u8>::from_hex(&serialized_tx).unwrap();
        let mut bytes_slice = &bytes[..];
        //println!("{:#?}", bytes_slice);
        let (version, version_hex) = Self::read_4_bytes(&mut bytes_slice).unwrap();
        TxFormat::paint(&version_hex, Color::BrightBlue, false, false);
        let input_count = Self::read_varint_bytes(&mut bytes_slice, DARK_ORANGE).unwrap();
        let inputs = Self::read_inputs(&mut bytes_slice, input_count, BITCOIN_COLOR);
        let output_count = Self::read_varint_bytes(&mut bytes_slice, Color::Green).unwrap();
        let outputs = Self::read_outputs(&mut bytes_slice, output_count, Color::Green);
        let (locktime, locktime_hex) = Self::read_4_bytes(&mut bytes_slice).unwrap();
        println!("{}", &locktime_hex.bright_blue());
        let transaction_id = Self::hash_raw_transaction(&bytes).unwrap();
        let parsed_tx = ParsedTransaction {
            version,
            inputs,
            outputs,
            locktime,
            transaction_id
        };
        Self::to_json(&parsed_tx)
    }

    fn to_json(parsed_tx: &ParsedTransaction) {
        let serialized = serde_json::to_string_pretty(&parsed_tx).unwrap();
        println!("{}", serialized);
    }

    fn read_4_bytes(stream: &mut &[u8]) -> TxParserResult<(u32, String)> {
        // Create a buffer of 4 bytes
        let mut buffer = [0; 4];
        // Read from our stream byte the requested buffer size
        stream.read(&mut buffer)?;
        Ok((
            u32::from_le_bytes(buffer), // Read the buffer in little endian
            hex::encode(buffer)
        )) 
    }

    fn read_8_bytes(stream: &mut &[u8]) -> TxParserResult<(u64, String)> {
        // Create a buffer of 4 bytes
        let mut buffer = [0; 8];
        // Read from our stream byte the requested buffer size
        stream.read(&mut buffer)?;
        Ok((
            u64::from_le_bytes(buffer), // Read the buffer in little endian
            hex::encode(buffer)
        )) 
    }

    // Programming bitcoin p92, chapter 5 
    fn read_varint_bytes(stream: &mut &[u8], color: Color) -> TxParserResult<u64> {
        // The first bytes defines the lenght of the varint
        let mut marker = [0; 1];
        stream.read(&mut marker)?;
        // The number that express the varint field
        let num: u64;
        let hex_varint: String;

        if marker[0] < 0xFD {
            num = u8::from_le_bytes(marker) as u64;
            hex_varint = hex::encode(marker);
        } else if marker[0] == 0xFD {
            let mut buffer = [0; 2];
            stream.read(&mut buffer)?;
            hex_varint = hex::encode(buffer);
            num = u16::from_le_bytes(buffer) as u64;
        } else if marker[0] == 0xFE {
            let mut buffer = [0; 4];
            stream.read(&mut buffer)?;
            hex_varint = hex::encode(buffer);
            num = u32::from_le_bytes(buffer) as u64;
        } else {
            let mut buffer = [0; 8];
            stream.read(&mut buffer)?;
            hex_varint = hex::encode(buffer);
            num = u64::from_le_bytes(buffer);
        }
        TxFormat::paint(&hex_varint, color, true, false);
        Ok(num)
    }

    fn read_inputs(stream: &mut &[u8], input_count: u64, color: Color) ->Vec<Input> {
        let mut inputs = vec![];
        for _ in 0 .. input_count {
            let txid = Self::read_transaction(stream, color).unwrap();
            let (vout, tx_index_hex) = Self::read_4_bytes(stream).unwrap();
            TxFormat::paint(&tx_index_hex, DARK_ORANGE, false, false);
            let script_sig_size = Self::read_varint_bytes(stream, color).unwrap();
            let script_sig = Self::read_specific_byte_length(stream, script_sig_size).unwrap();
            TxFormat::paint(&script_sig, color, false, false);
            //println!("Script Sig: {:#?}", script_sig);
            let (sequence, sequence_hex) = Self::read_4_bytes(stream).unwrap();
            TxFormat::paint(&sequence_hex, DARK_ORANGE, true, false);
            inputs.push(
                Input {
                    txid,
                    vout,
                    script_sig,
                    sequence
                }
            );
        }
        inputs
    }

    fn read_outputs(stream: &mut &[u8], output_count: u64, color: Color) -> Vec<Output> { 
        let mut outputs = vec![];
        for _ in 0 .. output_count {
            let (output_satoshis, output_satoshis_hex) = Self::read_8_bytes(stream).unwrap();
            TxFormat::paint(&output_satoshis_hex, color, false, true);
            let script_pub_key_length = Self::read_varint_bytes(stream, color).unwrap();
            let script_pub_key = Self::read_specific_byte_length(stream, script_pub_key_length).unwrap();
            TxFormat::paint(&script_pub_key, color, false, false);

            outputs.push(
                Output { 
                    amount: output_satoshis, 
                    script_pub_key: script_pub_key 
                }
            )
        }
        outputs
    }

    fn read_transaction(stream: &mut &[u8], color: Color) -> TxParserResult<[u8; 32]> {
        // The transaction id length is 32 bytes
        let mut buffer = [0; 32];
        stream.read(&mut buffer)?;
        TxFormat::paint(&hex::encode(buffer), color, false, false);
        // The representation of the txID is little endian but the TX ID is in big endian, so reverse
        buffer.reverse();
        Ok(buffer)
    }

    fn read_specific_byte_length(stream: &mut &[u8], length: u64) -> TxParserResult<String> {
        // Create a vector passing usize type variable
        let mut buffer = Vec::with_capacity(length as usize); 
        stream.read_buffer(&mut buffer)?;
        Ok(hex::encode(buffer))
    }

    // Double-sha256 hash of serialized transaction. Displayed in big-endian
    fn hash_raw_transaction(bytes: &[u8]) -> TxParserResult<[u8; 32]> {
        // Create a Sha256 object
        let mut hasher = Sha256::new();
        hasher.update(&bytes);
        let hash1 = hasher.finalize();
    
        let mut hasher = Sha256::new();
        hasher.update(hash1);
        let mut hash2 = hasher.finalize();
    
        // Reverse to display in big-endian
        hash2.reverse();
    
        Ok(<[u8; 32]>::from(hash2))
    }
}

