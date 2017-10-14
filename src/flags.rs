//Runtime Flags functionality namespace

//Used to decide what parts of the program will execute
pub struct Flags {
    pub prime_check: bool,
    pub get_factors: bool,
    pub prime_next: bool,
    pub prime_prev: bool,
    pub request_help: bool,
}

//Initializes Flags
pub fn init() -> Flags {
    Flags {
        prime_check: false,
        get_factors: false,
        prime_next: false,
        prime_prev: false,
        request_help: false,
    }
}

//Change flags based on received argument parameter
impl Flags {
    pub fn set_flag(&mut self, arg: &str) {
        match arg {
            "-c" => self.prime_check = true,
            "-f" => self.get_factors = true,
            "-n" => self.prime_next = true,
            "-p" => self.prime_prev = true,
            "-h" => self.request_help = true,
            "-a" => {
                self.prime_check = true;
                self.get_factors = true;
                self.prime_next = true;
                self.prime_prev = true;
            }
            _ => if arg.starts_with("-") {
                println!("Error: Invalid argument: {}", arg);
            },
        }
    }
}
