extern crate tiny_keccak;

use self::tiny_keccak::Keccak;

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