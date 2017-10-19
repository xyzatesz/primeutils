extern crate primeutils_lib;

use std::env::args;
use primeutils_lib::*;
use primeutils_lib::flags::Flags;

fn main() {
    //Initialize runtime variables
    let mut runtime_flags: Flags = flags::init();
    let mut number = 0;

    //Check if the number of parameters is 0 (the first is the program path)
    if args().count() == 1 {
        help::print_help_short();
        return;
    }

    //Set runtime flags and parse input number
    for arg in args() {
        runtime_flags.set_flag(&arg);
        if arg.parse::<usize>().is_ok() {
            number = match arg.parse::<usize>() {
                Ok(val) => val,
                Err(_) => return
            }
        }
    }

    //Check for help request first
    if runtime_flags.request_help == true {
        help::print_help();
        return;
    }

    //Check for input validity (skip if --gen is supplied)
    if number < 2 && runtime_flags.generate_prime != true {
        println!("Error: Invalid number input");
        return;
    }

    //If only a number, thus no flags are supplied, execute prime_check() (skip if --gen is supplied)
    if args().count() == 2 && runtime_flags.generate_prime != true {
        utils::check_prime(number);
        return;
    }

    //Execute requested functions
    if runtime_flags.generate_prime == true {
        utils::generate_prime(runtime_flags.gen_prime_min, runtime_flags.gen_prime_max);
        return;
    }
    if runtime_flags.prime_check == true {utils::check_prime(number);}
    if runtime_flags.prime_next == true {utils::prime_next(number);}
    if runtime_flags.prime_prev == true {utils::prime_prev(number);}
    if runtime_flags.get_factors == true {utils::get_factors(number);}
}
