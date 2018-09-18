use std::fmt;
use EthereumKeySet;
use rustc_hex::ToHex;
use keccak::Keccak256;
use self::key::PublicKey;
use ethereum_types::Address;
use secp256k1::Error as SecpError;
use secp256k1::{Secp256k1, Message, key};

pub struct SignedMessage {
    msg: String,
    addr: Address,
    key: PublicKey,
    sig: [u8;65]
}

impl fmt::Display for SignedMessage {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		writeln!(f, "Signed message:\n{}", self.msg)?;
		writeln!(f, "\nSigning key:\n{:?}", self.key)?;
        writeln!(f, "\nEthereum Address\n{:?}", self.addr)?;
		write!(f, "\nSignature: \n{}", self.sig.to_hex())
	}
}

impl SignedMessage {
    pub fn new(msg: &str, keyset: EthereumKeySet) -> Result<SignedMessage, SecpError> {
        let sig = sign_message(hash_message(&msg), &keyset)?;
        Ok(SignedMessage{msg: msg.to_string(), key: keyset.public, addr: keyset.address, sig})
    }
}

fn hash_message(msg: &str) -> [u8;32] {
    msg.as_bytes().keccak256()
}

// Note: See issues here on standardizing of sigs. 
// https://github.com/paritytech/parity-ethereum/issues/5490
fn sign_message(hashed_msg: [u8;32], keyset: &EthereumKeySet) -> Result<[u8;65], SecpError> {
    let message = Message::from_slice(&hashed_msg).expect("32 bytes");
    let secp_context = Secp256k1::new();
    match secp_context.sign_recoverable(&message, &keyset.secret) {
        Ok(sig) => {
            let (rec_id, data) = sig.serialize_compact(&secp_context);
            let mut data_arr = [0; 65];
            data_arr[0..64].copy_from_slice(&data[0..64]);
            data_arr[64] = rec_id.to_i32() as u8;
            Ok(data_arr)
        }
        Err(e) => Err(e)
    }
}

// TODO: Write something to verify a msg?