extern crate rand;
extern crate docopt;
extern crate secp256k1;
extern crate rustc_hex;
extern crate threadpool;
extern crate tiny_keccak;
extern crate ethereum_types;

use std::sync;
//use std::result;
//use docopt::Docopt;
use rustc_hex::FromHex;
use tiny_keccak::Keccak;
use rand::{OsRng, Rng, thread_rng};
use secp256k1::{Secp256k1, Message, Error, key};
use ethereum_types::{Address, Public, Secret};

/*
 * TODO: Import public & private from key to stop using key:: everywhere!
 * TODO: Switch to using result and make compositions with that!!
 * TODO: Can I compose/pipe in Rust?
 * TODO: Can I call funcs. first class WITH args?
 * TODO: Can I curry functions?
 * TODO: Make a new type to hold the key structure plus derivation logic.
 * TODO: Rm the result suffixes
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

fn log_monad_contents<T>(monad: T) -> T 
    where T: std::fmt::Debug
{
    println!("[+] {:?}", monad);
    monad
}

fn main() {
    /*
    let thread_priv = generate_vanity_priv_key_threaded("00");
    let thread_addr = private_key_to_eth_addr(thread_priv);
    println!("{:?}\n{:?}", thread_priv, thread_addr);
    */
    match generate_random_priv_key_result()
        .map(log_monad_contents)
        .and_then(get_public_key_from_secret_result) // and_then == chain/bind
        .map(log_monad_contents)
        .map(public_key_to_long_eth_addr) // TODO: combine?
        .map(public_key_to_address_result) {
            Ok(r)  => println!("[+] Ethereum Address: {:?}",r),
            Err(e) => println!("[-] Error: {:?}",e)
        };
}

//pub fn generate_key_set() -> Ethereum_Key_Set {
    // TODO: use above pipeline and create the struct to make this func work!
    // TODO: create a formatter to print the struct contents
    // TODO: create a getter for the individual keys
    // TOOD: Implement something that returns this struct in the vanity stuff!
//}

pub fn generate_oraclize_address() -> key::SecretKey {
    generate_vanity_priv_key("0000")
}

pub fn generate_vanity_priv_key(prefix: &str) -> key::SecretKey {
    let priv_key = generate_random_priv_key();
    if starts_with_prefix(priv_key, &prefix.from_hex().expect("Error: valid hex required for prefix!")) {
        priv_key
    } else {
        generate_vanity_priv_key(prefix)
    }
}

pub fn starts_with_prefix(secret: key::SecretKey, prefix: &Vec<u8>) -> bool {
    private_key_to_eth_addr(secret).starts_with(&prefix)
}

pub fn private_key_to_eth_addr(secret: key::SecretKey) -> Address {
    public_key_to_address(&public_key_to_long_eth_addr(get_public_key_from_secret(secret))) // TODO: compose!!
}


pub fn generate_random_priv_key() -> key::SecretKey {
    key::SecretKey::from_slice(&Secp256k1::new(), &get_32_random_bytes_arr()).expect("Failed to generate secret key")
}

pub fn generate_random_priv_key_result() -> Result<key::SecretKey, Error> {
    key::SecretKey::from_slice(&Secp256k1::new(), &get_32_random_bytes_arr())
}

pub fn get_32_random_bytes_arr() -> [u8;32] {
    let mut arr = [0; 32];
    arr.copy_from_slice(&get_x_random_bytes_vec(32));
    arr
}

pub fn get_x_random_bytes_vec(len: usize) -> Vec<u8> {
    let mut x = vec![0u8; len]; 
    thread_rng().fill_bytes(&mut x);
    x
}
pub fn get_public_key_from_secret(secret_key: key::SecretKey) -> key::PublicKey {
    key::PublicKey::from_secret_key(&Secp256k1::new(), &secret_key).expect("Failed to derive public key!")
}

pub fn get_public_key_from_secret_result(secret_key: key::SecretKey) -> Result<key::PublicKey, Error> {
    key::PublicKey::from_secret_key(&Secp256k1::new(), &secret_key)
}

pub fn public_key_to_address(public: &Public) -> Address {
    let hash = public.keccak256();      // Can call keccak on this because the keccak trait accommodates a 32 byte u8 arr.
    let mut result = Address::default();
    result.copy_from_slice(&hash[12..]); // Pub addr. is last 20 bytes of the hashed public key.
    result
}

pub fn public_key_to_address_result(public: Public) -> Address {
    let hash = public.keccak256();      // Can call keccak on this because the keccak trait accommodates a 32 byte u8 arr.
    let mut result = Address::default();
    result.copy_from_slice(&hash[12..]); // Pub addr. is last 20 bytes of the hashed public key.
    result
}

pub fn public_key_to_long_eth_addr(pub_key: key::PublicKey) -> Public {
    let context = secp256k1::Secp256k1::new();
    let serialized = pub_key.serialize_vec(&context, false);
    let mut public = Public::default();
    public.copy_from_slice(&serialized[1..65]);
    public
}

// TODO: implement a version that will hash longer input.
pub trait Keccak256<T> {
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

fn generate_vanity_priv_key_threaded(prefix: &'static str) -> key::SecretKey {
    let pool = threadpool::Builder::new().build();
    let (tx, rx) = sync::mpsc::sync_channel(1);
    for _ in 0..pool.max_count() {
        let tx = tx.clone();
        pool.execute(move || {
            let pref = prefix.from_hex().expect("Error: valid hex required for prefix!");
            loop {
                let priv_key = generate_random_priv_key();
                if !starts_with_prefix(priv_key, &pref) {
                    continue;
                }
                tx.send(priv_key).expect("Error sending private key from thread.");
            }
        });
    };
    rx.recv().expect("No fitting private key found!")
}

