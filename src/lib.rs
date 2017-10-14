pub mod flags {
    //Used to decide what parts of the program will execute
    pub struct Flags {
        prime_check: bool,
        get_factors: bool,
        prime_above: bool,
        prime_below: bool
    }

    //Initializes Flags
    pub fn init() -> Flags {
        Flags {
            prime_check: true,
            get_factors: false,
            prime_above: false,
            prime_below: false
        }
    }

    //Change flags based on received argument parameter
    impl Flags {
        pub fn set_flag(&mut self, str arg) {
            match arg {
                "-h" => (); //TODO IMPLEMENT HELP
                "-f" => (); //TODO IMPLEMENT PRIME FACTOR PRINTING
                "-a" => (); //TODO IMPLEMENT PRIME ABOVE
                "-b" => (); //TODO IMPLEMENT PRIME BELOW
                _ => ();    //TODO PRINT INVALID ARGUMENT
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}