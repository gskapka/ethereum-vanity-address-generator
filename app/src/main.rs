extern crate ethkeygen;

fn main() {
    match ethkeygen::generate_vanity_priv_key_threaded("00") { // Note, using result thusly is NOT lazy!
        Ok(r)  => println!("Secret: {:?}", r),
        Err(e) => println!("Error: {}",e)
    };
}
