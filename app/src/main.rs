extern crate ethkeygen;

fn main() {
    // let keyset = match ethkeygen::EthereumKeySet::new_vanity("00") {
    //     Ok(k)  => k,
    //     Err(e) => panic!("Error generating keyset: {:?}", e)
    // };
    // println!("{}", keyset);
    // keyset.unsafe_show_secret();

    let keyset = ethkeygen::EthereumKeySet::new().unwrap(); // Note: unsafe!
    let msg = String::from("This is a message to be hashed");
    let hash = ethkeygen::hash_message(&msg);
    let sig = ethkeygen::sign_message(hash, keyset);
    println!("Hash must have checked out!");
    // println!("{:?}", hash);
    // assert_eq!(hash, "187bf203fc3763839208637560caad821511e24bb3f798f98d178e181ac4b773");
}
