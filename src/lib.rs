extern crate rand;
extern crate secp256k1;
extern crate rustc_hex;
extern crate threadpool;
extern crate tiny_keccak;
extern crate ethereum_types;

mod utils;
mod macros;

use rustc_hex::FromHex;
use tiny_keccak::Keccak;
use secp256k1::Secp256k1;
use rand::{Rng, thread_rng};
use utils::log_monad_contents;
use std::sync::mpsc::sync_channel;
use secp256k1::Error as SecpError;
use ethereum_types::{Address, Public};
use secp256k1::key::{SecretKey, PublicKey};

/*
 *
 * TODO: Clean up the _result suffix stuff!
 * TODO: Can I compose/pipe in Rust? - Yes, implement.
 * TODO: Can I call funcs. first class WITH args? - Only via closures :( Ugly
 * TODO: Can I curry functions? - See above
 * TODO: Make a new type to hold the key structure plus derivation logic.
 * TODO: Put monad logger in utils
 * TODO: Factor our the keccak hasher into it's own file
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

/*
let message = Message::from_slice(&[0xab; 32]).expect("32 bytes");
let sig = secp.sign(&message, &secret_key);
assert!(secp.verify(&message, &sig, &public_key).is_ok());
*/

fn generate_key_set() -> Result<Address, SecpError> {//Ethereum_Key_Set {
    // TODO: use above pipeline and create the struct to make this func work!
    // TODO: create a formatter to print the struct contents
    // TODO: create a getter for the individual keys
    // TODO: Implement something that returns this struct in the vanity stuff!
    generate_random_priv_key_result()
        .map(log_monad_contents)
        .and_then(get_public_key_from_secret_result) // and_then == chain/bind
        .map(log_monad_contents)
        .map(public_key_to_long_eth_addr) // TODO: combine?
        .map(public_key_to_address_result)
}

fn generate_oraclize_address() -> SecretKey {
    generate_vanity_priv_key("0000")
}

fn generate_vanity_priv_key(prefix: &str) -> SecretKey {
    match generate_random_priv_key_result() {
        Ok(k) => {
            if starts_with_prefix(k, &prefix.from_hex().expect("Error: valid hex required for prefix!")) {
                k
            } else {
                generate_vanity_priv_key(prefix)
            }
        },
        Err(_) => panic!("Error generating random secret!")
    }
}

fn starts_with_prefix(secret: SecretKey, prefix: &Vec<u8>) -> bool {
    private_key_to_eth_addr(secret).starts_with(&prefix)
}

fn private_key_to_eth_addr(secret: SecretKey) -> Address {
    public_key_to_address(&public_key_to_long_eth_addr(get_public_key_from_secret(secret))) // TODO: compose!!
}

fn generate_random_priv_key_result() -> Result<SecretKey, SecpError> {
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
fn get_public_key_from_secret(secret_key: SecretKey) -> PublicKey {
    PublicKey::from_secret_key(&Secp256k1::new(), &secret_key).expect("Failed to derive public key!")
}

fn get_public_key_from_secret_result(secret_key: SecretKey) -> Result<PublicKey, SecpError> {
    PublicKey::from_secret_key(&Secp256k1::new(), &secret_key)
}

fn public_key_to_address(public: &Public) -> Address {
    let hash = public.keccak256();      // Can call keccak on this because the keccak trait accommodates a 32 byte u8 arr.
    let mut result = Address::default();
    result.copy_from_slice(&hash[12..]); // Pub addr. is last 20 bytes of the hashed public key.
    result
}

fn public_key_to_address_result(public: Public) -> Address {
    let hash = public.keccak256();      // Can call keccak on this because the keccak trait accommodates a 32 byte u8 arr.
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

// TODO: implement a version that will hash longer input.
trait Keccak256<T> {
    fn keccak256(&self) -> T where T: Sized;      // Takes any type that implements the 'Sized' typeclass.
}

impl Keccak256<[u8; 32]> for [u8] {               // Takes arr of length 32 & type u8, returns [u8]
    fn keccak256(&self) -> [u8; 32] {
        let mut keccak = Keccak::new_keccak256(); // Get hash func. from struct in crate
        let mut result = [0u8; 32];               // make arr. 32 long of u8 zeroes. (Sugar for 
                                                  // `let must result [u8; 32] = [0; 32];`)
        keccak.update(self);                      // Add self param to hash func.(Can add more using 
                                                  // byte literals: `b" "` etc)
                                                  // Self is the type of the current object, used
                                                  // here in impl where it's stand in for whatever
                                                  // type ends up implementing the keccak trait.
        keccak.finalize(&mut result);             // Finalizes the hash, folds it, pads it etc.
        result
    }
}

pub fn generate_vanity_priv_key_threaded_result(prefix: &'static str) -> Result<SecretKey, String> {
    let pool = threadpool::Builder::new().build();
    let (tx, rx) = sync_channel(1);
    for _ in 0..pool.max_count() {
        let tx = tx.clone();
        pool.execute(move || {
            let pref = prefix.from_hex().expect("Error: valid hex required for prefix!");
            loop { // Note: Used recursion the first time but no tail-call recursion optimization in Rust :(
                match generate_random_priv_key_result() {
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
    match rx.recv() {
        Ok(k)  => Ok(k),
        Err(_) => Err(String::from("Error receiving secret from thread!"))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
