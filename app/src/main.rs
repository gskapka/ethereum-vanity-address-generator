extern crate ethkeygen;

fn main() {
    // match ethkeygen::generate_vanity_priv_key_threaded("00") { // Note:, using result thusly is NOT lazy!
    //     Ok(r)  => println!("Secret: {:?}", r),
    //     Err(e) => println!("Error: {}",e)
    // };
    // let keyset = ethkeygen::EthereumKeySet::new_vanity_addr("00");
    let keyset = ethkeygen::EthereumKeySet::new();
    println!("{}", keyset);
    keyset.unsafe_show_secret();
}
