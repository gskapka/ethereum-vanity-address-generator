extern crate secp256k1;

use std::fmt;
use EthereumKeySet;
use keccak::Keccak256;
use ethereum_types::Address;
use secp256k1::Error as SecpError;
use self::key::{SecretKey, PublicKey};
use secp256k1::{Secp256k1, Message, RecoverableSignature, key};

pub struct SignedMessage {
    msg: String,
    addr: Address,
    key: PublicKey,
    sig: RecoverableSignature
}

impl fmt::Display for SignedMessage {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		writeln!(f, "Signed message:\n{}", self.msg)?;
		writeln!(f, "\nSigning key:\n{:?}", self.key)?;
        writeln!(f, "\nEthereum Address\n{:?}", self.addr)?;
		write!(f, "\nSignature: \n{:?}", self.sig)
	}
}

impl SignedMessage {
    pub fn new(msg: &str, keyset: EthereumKeySet) -> Result<SignedMessage, SecpError> {
        let sig = sign_message(hash_message(&msg), &keyset)?;
        Ok(SignedMessage{msg: msg.to_string(), key: keyset.public, addr: keyset.address, sig})
    }
}

// fn prefix_message<'a>(msg: &'a str) -> &'a str {
//     // let prefix: &str = "\x19Ethereum Signed Message:\n";
//     // let len: &str = &msg.len().to_string();
//     let mut final_str = String::new();
//     final_str.push("\x19Ethereum Signed Message:\n");
//     final_str.push(&msg.len().to_string());
//     final_str.push(msg);
//     final_str
// }

fn hash_message(msg: &str) -> [u8;32] {
    msg.as_bytes().keccak256()
}

fn sign_message(hashed_msg: [u8;32], keyset: &EthereumKeySet) -> Result<RecoverableSignature, SecpError> {
    let message = Message::from_slice(&hashed_msg).expect("32 bytes");
    let secp = Secp256k1::new(); // Create a signing capable context
    match secp.sign_recoverable(&message, &keyset.secret) {
        Ok(sig) => {
            // assert!(secp.verify(&message, &sig, &keyset.public).is_ok());
            Ok(sig)
        }
        Err(e) => Err(e)
    }
}