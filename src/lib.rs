extern crate rand;
extern crate secp256k1;
extern crate rustc_hex;
extern crate threadpool;
extern crate ethereum_types;

mod sign;
mod keccak;

use std::fmt;
use keccak::Keccak256;
use rustc_hex::FromHex;
use secp256k1::Secp256k1;
pub use sign::SignedMessage;
use rand::{Rng, thread_rng};
use secp256k1::Error as SecpError;
use ethereum_types::{Address, Public};
use secp256k1::key::{SecretKey, PublicKey};

/*
 *
 * TODO: Factor out the key creation too & just have this lib as an entry point!
 * TODO: Implement message signing on the struct! 
 * TODO: Can I compose/pipe in Rust? - Yes, implement.
 * TODO: Can I call funcs. first class WITH args? - Only via closures :( Ugly
 * TODO: Can I curry functions? - See above
 * TODO: Rm panics!
 * TODO: Error handle better so I can ? everywhere for terseness.
 * 
 * The goal is to generate a private key (with 4 0's maybe?) and then seal 
 * that in the enclave, after first reporting out the enclave what the 
 * public key & derived ethereum address is.
 *
 * Then next can get access the same enclave to sign a message and return
 * the signed message proving that is still has access to the same sealed
 * private key.
 *
 * When signing any arbritrary message, we first hash it. Here we'll use
 * the keccak hasher we already have access to. Hash, sign, console log
 * the signature.
 *
 * To verify, we use the PUBLIC KEY, NOT THE ETHEREUM ADDR! So we SIGN 
 * it with the priv, but can verify with the PUB, ∴ proving we own the 
 * corresponding priv key.
 *
 * So maybe the app, not the enclave, should have a tool to verify the
 * messages? All the enc. needs to be able to do is to sign the msgs.
 *
 * Could maybe have the CLI accept a message in utf, either pasted in
 * or read from a file (docopt? - add it now whilst online) and have
 * the enc sign it. (How to pipe to enc? Interesting! Secure channel
 * required!) 
 *
 * */

pub struct EthereumKeySet {
    secret: SecretKey,
    public: PublicKey,
    address: Address
}

impl fmt::Display for EthereumKeySet {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		writeln!(f, "Private key: [redacted - please use `unsafe_show_secret` to view]")?;
		writeln!(f, "{:?}", self.public)?;
		write!(f, "Ethereum Address: {:?}", self.address)
	}
}

impl EthereumKeySet {

    pub fn new() -> Result<EthereumKeySet, SecpError> {
        let s = generate_random_priv_key()?;
        let p = get_public_key_from_secret(s)?;
        let a = public_key_to_address(public_key_to_long_eth_addr(p));
        Ok(EthereumKeySet{secret: s, public: p, address: a})
    }

    pub fn new_vanity(prefix: &'static str) -> Result<EthereumKeySet, SecpError> {
        let s = generate_vanity_priv_key_threaded(prefix).unwrap();
        let p = get_public_key_from_secret(s)?;
        let a = public_key_to_address(public_key_to_long_eth_addr(p));
        Ok(EthereumKeySet{secret: s, public: p, address: a})
    }

    pub fn unsafe_show_secret(&self) {
        println!("{:?}", self.secret);
    }
}

fn starts_with_prefix(secret: SecretKey, prefix: &Vec<u8>) -> bool {
    private_key_to_eth_addr(secret).starts_with(&prefix)
}

fn private_key_to_eth_addr(secret: SecretKey) -> Address {
    match get_public_key_from_secret(secret) {
        Ok(k)  => public_key_to_address(public_key_to_long_eth_addr(k)),
        Err(_) => panic!("Error getting public key from secret!")
    }
}

fn generate_random_priv_key() -> Result<SecretKey, SecpError> {
    SecretKey::from_slice(&Secp256k1::new(), &get_32_random_bytes_arr())
}

fn get_32_random_bytes_arr() -> [u8;32] {
    let mut arr = [0; 32];
    arr.copy_from_slice(&get_x_random_bytes_vec(32));
    arr
}

fn get_x_random_bytes_vec(len: usize) -> Vec<u8> {
    let mut x = vec![0u8; len]; 
    thread_rng().fill_bytes(&mut x);
    x
}

fn get_public_key_from_secret(secret_key: SecretKey) -> Result<PublicKey, SecpError> {
    PublicKey::from_secret_key(&Secp256k1::new(), &secret_key)
}

fn public_key_to_address(public: Public) -> Address {
    let hash = public.keccak256();
    let mut result = Address::default();
    result.copy_from_slice(&hash[12..]); // Pub addr. is last 20 bytes of the hashed public key.
    result
}

fn public_key_to_long_eth_addr(pub_key: PublicKey) -> Public {
    let context = secp256k1::Secp256k1::new();
    let serialized = pub_key.serialize_vec(&context, false);
    let mut public = Public::default();
    public.copy_from_slice(&serialized[1..65]);
    public
}

fn generate_vanity_priv_key_threaded(prefix: &'static str) -> Result<SecretKey, std::sync::mpsc::RecvError> {
    let pool = threadpool::Builder::new().build();
    let (tx, rx) = std::sync::mpsc::sync_channel(1);
    for _ in 0..pool.max_count() {
        let tx = tx.clone();
        pool.execute(move || {
            let pref = prefix.from_hex().expect("Error: valid hex required for prefix!");
            loop { // Note: Used recursion the first time but no tail recursion in Rust :(
                match generate_random_priv_key() {
                    Ok(k)  => {
                        if !starts_with_prefix(k, &pref) {
                            continue;
                        }
                        tx.send(k).expect("Error sending secret from thread!")
                    },
                    Err(_) => panic!("Error generating random secret in thread!")
                };
            }
        });
    };
    rx.recv()
}

// #[cfg(test)] // TODO: Some tests :P
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
