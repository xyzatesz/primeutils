//Runtime Flags functionality namespace
pub mod flags {
    //Used to decide what parts of the program will execute
    pub struct Flags {
        pub prime_check: bool,
        pub get_factors: bool,
        pub prime_next: bool,
        pub prime_prev: bool,
        pub request_help: bool
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
                _ => {
                    if arg.starts_with("-") {
                        println!("Error: Invalid argument: {}", arg);
                    }
                }
            }
        }
    }
}

//Prime Utilities functionality namespace
pub mod utils {
    //Check if number is prime or composite, and return boolean
    fn is_prime(number: usize) -> bool {
        let root: i32 = (number as f64).sqrt() as i32 + 1;
        for i in 2..root {
            if number % (i as usize) == 0 {
                return false;
            }
        }
        true
    }

    //Check if number is prime or composite, and produce output
    pub fn check_prime(number: usize) {
        if is_prime(number) {
            println!("{} is a prime", number);
        } else {
            println!("{} is composite", number);
        }
    }

    //Find the closest prime above number
    pub fn prime_next(mut number: usize) {
        number = number + 1;
        while is_prime(number) != true {
            number = number + 1;
        }
        println!("The next prime number is {}", number);
    }

    //Find the closest prime below number
    pub fn prime_prev(mut number: usize) {
        number = number - 1;
        while is_prime(number) != true {
            number = number - 1;
        }
        println!("The previous prime number is {}", number);
    }

    //Output the prime factors of number
    pub fn get_factors(mut number: usize) {
        let mut factors: Vec<usize> = Vec::new();
        while number != 1 {
            //(number + 1) is used, as the uppermost item is excluded
            for i in 2..(number + 1) {
                //If number is prime, divide with it, and exit the loop
                //Serves as a speed enhancement for large prime factors
                if is_prime(number) {
                    factors.push(number);
                    number = 1;
                    break;
                }
                //Else check if number is divisible by i, if yes, add it
                //to the list of factors and divide with it. Repeat.
                if number % (i as usize) == 0 {
                    number = number / (i as usize);
                    factors.push(i);
                    break;
                }
            }
        }
        //The factor checking always works on the previous number in the array,
        //so an extra number is added, which will not be evaluated
        factors.push(0);
        //Factor checking - Create output string from factor array
        let mut output = String::from("Prime factorization:");
        let mut prev = 0;
        let mut factor_exp = 1;
        //Always check if the current number is the same as the previous - if yes,
        //increment exponent, shift prev, and repeat. If no, concat number (prev) and
        //exponent to output string. Reset variables.
        for n in factors {
            if n == prev {
                factor_exp = factor_exp + 1;
                prev = n;
                continue;
            }
            if factor_exp > 1 {
                output = format!("{} {} ({}),", output, prev, factor_exp);
                prev = n;
                factor_exp = 1;
                continue;
            }
            //Happens at the first item
            if prev == 0 {
                prev = n;
                continue;
            }
            output = format!("{} {},", output, prev);
            prev = n;
        }
        println!("{}", &output[..(output.chars().count()-1)]);
    }
}

pub mod help {
    pub fn print_help() {
        println!("Usage: primeutils INTEGER [OPTIONS]");
        println!("       OPTIONS can be the following:");
        println!("           -c    Check INTEGER for compositeness");
        println!("           -n    Print the closest prime above INTEGER");
        println!("           -p    Print the closest prime below INTEGER");
        println!("           -f    Print the prime factorization of INTEGER");
        println!("           -a    Print everything");
        println!("           -h    Display this help screen");
        println!("       If options are omitted, -c is automatically assumed");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}