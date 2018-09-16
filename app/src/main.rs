extern crate ethkeygen;

use ethkeygen::generate_vanity_priv_key_threaded_result;

fn main() {
    match generate_vanity_priv_key_threaded_result("00") { // Note, using result thusly is NOT lazy!
        Ok(r)  => println!("Secret: {:?}", r),
        Err(e) => println!("Error: {}",e)
    };
}
