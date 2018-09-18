extern crate rand;
extern crate secp256k1;
extern crate rustc_hex;
extern crate threadpool;
extern crate ethereum_types;
pub mod sign;
pub mod keccak;
pub mod keygen;
pub use sign::SignedMessage;
pub use keygen::EthereumKeySet;
/*
 *
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