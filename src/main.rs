extern crate primeutils_lib;

use std::env::args;
use primeutils_lib::*;
use primeutils_lib::flags::Flags;

fn main() {
    //Initialize and set runtime flags
    let mut runtime_flags: Flags = flags::init();
    for arg in args() {
        
    }
}
