extern crate secp256k1;

use EthereumKeySet;
use keccak::Keccak256;
use secp256k1::{Secp256k1, Message};

pub fn hash_message(msg: &String) -> [u8;32] {
    msg.as_bytes().keccak256()
}

pub fn sign_message(hashed_msg: [u8;32], keyset: EthereumKeySet) {
    let message = Message::from_slice(&hashed_msg).expect("32 bytes");
    let secp = Secp256k1::new();
    let sig = secp.sign(&message, &keyset.secret).unwrap(); // FIXME: will panic!
    assert!(secp.verify(&message, &sig, &keyset.public).is_ok());
}