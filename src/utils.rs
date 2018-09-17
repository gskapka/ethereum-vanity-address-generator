use std::fmt::Debug;

// Simple monad logger. Logs contents & returns monad unmolested.
pub fn log_monad_contents<T>(m: T) -> T 
    where T: Debug
{
    println!("{:?}", m);
    m
}