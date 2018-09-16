// Thanks to Jan Nils Ferner @ 
// https://stackoverflow.com/questions/45786955/how-to-compose-functions-in-rust

macro_rules! compose {
    ( $last:expr ) => { $last };
    ( $head:expr, $($tail:expr), +) => {
        compose_two($head, compose!($($tail),+))
    };
}

fn compose_two<A, B, C, G, F>(f: F, g: G) -> impl Fn(A) -> C
    where F: Fn(A) -> B,
          G: Fn(B) -> C
{
    move |x| g(f(x))
}