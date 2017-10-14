pub mod flags {
    //Used to decide what parts of the program will execute
    pub struct Flags {
        prime_check: bool,
        get_factors: bool,
        prime_next: bool,
        prime_prev: bool,
        request_help: bool
    }

    //Initializes Flags
    pub fn init() -> Flags {
        Flags {
            prime_check: false,
            get_factors: false,
            prime_next: false,
            prime_prev: false,
            request_help: false
        }
    }

    //Change flags based on received argument parameter
    impl Flags {
        pub fn set_flag(&mut self, arg: &str) {
            match arg {
                "-c" => slef.prime_check = true,
                "-f" => self.get_factors = true,
                "-n" => self.prime_next = true,
                "-p" => self.prime_prev = true,
                "-h" => self.request_help = true,
                "-a" => {
                    self.prime_check = true,
                    self.get_factors = true,
                    self.prime_next = true,
                    self.prime_prev = true
                }
                _(param) => println!("Error: Invalid argument: {}", param)
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