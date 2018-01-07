//Runtime Flags functionality namespace

//Used to decide what parts of the program will execute
pub struct Flags {
    pub prime_check: bool,
    pub get_factors: bool,
    pub prime_next: bool,
    pub prime_prev: bool,
    pub request_help: bool,
    pub generate_prime: bool,
    pub gen_prime_min: u32,
    pub gen_prime_max: u32,
}

//Initializes Flags
pub fn init() -> Flags {
    Flags {
        prime_check: false,
        get_factors: false,
        prime_next: false,
        prime_prev: false,
        request_help: false,
        generate_prime: false,
        gen_prime_min: 2,
        gen_prime_max: <u32>::max_value(),
    }
}

//Change flags based on received argument parameter
impl Flags {
    pub fn set_flag(&mut self, arg: &str) {
        //If the length of the word is >2 (not shorthand), try these
        if arg.len() > 2 || self.generate_prime == true {
            //Set generator subroutine - disable normal behavior
            if arg.eq("--gen") {
                self.generate_prime = true;
                self.prime_check = false;
                self.get_factors = false;
                self.prime_next = false;
                self.prime_prev = false;
            } else if arg.starts_with("--min") {
                let mut copy: String = String::from(arg);
                copy = copy.split_off(6);
                self.gen_prime_min = match copy.parse::<u32>() {
                    Ok(val) => val,
                    Err(why) => {
                        println!("Error: Invalid --min argument: {}", why);
                        return
                    }
                }
            } else if arg.starts_with("--max") {
                let mut copy: String = String::from(arg);
                copy = copy.split_off(6);
                self.gen_prime_max = match copy.parse::<u32>() {
                    Ok(val) => val,
                    Err(why) => {
                        println!("Error: Invalid --max argument: {}", why);
                        return
                    }
                }
            } else if arg.starts_with("-") {
                println!("Error: Invalid argument: {}", arg);
            }
        } else {
            match arg {
                "-c" => self.prime_check = true,
                "-f" => self.get_factors = true,
                "-a" => self.prime_next = true,
                "-b" => self.prime_prev = true,
                "-h" => self.request_help = true,
                "-e" => {
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
}
