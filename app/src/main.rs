extern crate ethkeygen;

fn main() {
    let keyset = match ethkeygen::EthereumKeySet::new_vanity("00") {
        Ok(k)  => k,
        Err(e) => panic!("Error generating keyset: {:?}", e)
    };
    println!("{}", keyset);
    keyset.unsafe_show_secret();
}
