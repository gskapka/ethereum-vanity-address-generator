use std;

// Simple monad logger. Logs contents & returns monad unmolested.
pub fn log_monad_contents<T>(m: T) -> T 
    where T: std::fmt::Debug
{
    println!("{:?}", m);
    m
}