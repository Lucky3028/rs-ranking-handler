use std::io::{self, prelude::*};

pub fn pause() {
    print!("Press Enter key to continue...");
    io::stdout().flush().unwrap();
    io::stdin().read_exact(&mut [0u8]).unwrap();
}
