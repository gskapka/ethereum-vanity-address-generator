extern crate ethkeygen;

fn main() {
    let keyset = ethkeygen::EthereumKeySet::new().unwrap(); // Note: unsafe!
    keyset.unsafe_show_secret();
    let msg = "This is a message to be hashed";
    match ethkeygen::SignedMessage::new(msg, keyset) {
        Ok(thing) => println!("{}", thing),
        Err(e) => panic!("Error getting signed message!")
    }
}
