extern crate primeutils_lib;

use std::env::args;
use primeutils_lib::*;
use primeutils_lib::flags::Flags;

fn main() {
    //Initialize runtime variables
    let mut runtime_flags: Flags = flags::init();
    let number = 0;

    //Check if the number of parameters is 0 (the first is the program path)
    if args().count() == 1 {
        //TODO IMPLEMENT HELP
        return;
    }

    //Set runtime flags and parse input number
    for arg in args() {
        runtime_flags.set_flag(arg);
        if arg.parse::<usize>().is_ok() {
            number = match arg.parse::<usize>() {
                Ok(val) => val,
                Err => return
            }
        }
    }

    //Check for input validity
    if number < 2 {
        println!("Error: Invalid number input");
        return;
    }
}
