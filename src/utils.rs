//Prime Utilities functionality namespace

extern crate rand;

//Check if number is prime or composite, and return boolean
fn is_prime(number: u32) -> bool {
    let root: u32 = (number as f64).sqrt() as u32 + 1;
    for i in 2..root {
        if number % (i as u32) == 0 {
            return false;
        }
    }
    true
}

//Check if number is prime or composite, and produce output
pub fn check_prime(number: u32) {
    if is_prime(number) {
        println!("{} is a prime", number);
    } else {
        println!("{} is not a prime", number);
    }
}

//Find the closest prime above number
pub fn prime_next(mut number: u32) {
    let backup = number;
    number = number + 1;
    while is_prime(number) != true {
        number = number + 1;
    }
    println!("The closest prime number above {} is {}", backup, number);
}

//Find the closest prime below number
pub fn prime_prev(mut number: u32) {
    let backup = number;
    number = number - 1;
    while is_prime(number) != true {
        number = number - 1;
    }
    println!("The closest prime number below {} is {}", backup, number);
}

//Output the prime factors of number
pub fn get_factors(mut number: u32) {
    let mut factors: Vec<u32> = Vec::new();
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
            if number % (i as u32) == 0 {
                number = (number / (i as u32)) as u32;
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
    let mut prev: u32 = 0;
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
    println!("{}", &output[..(output.chars().count() - 1)]);
}

//Generate a random prime number between arguments
pub fn generate_prime(min: u32, max: u32) {
    //Error checking: Correct parameters
    if min >= max {
        println!("Error: --min value should be lower than --max value");
        return;
    }
    if min < 2 || max < 3 {
        println!("Error: Invalid interval supplied");
        return;
    }
    //Error checking: (Minimal) Prime exists in interval
    if (min + 100) >= max {
        let mut bad_interval = true;
        for i in min..max {
            if is_prime(i) {bad_interval = false;}
        }
        if bad_interval {
            println!("Error: No primes detected in supplied interval");
            return;
        }
    }
    let mut number: u32 = 4;
    while !is_prime(number as u32) || number < 2 {
        number = rand::random::<u32>() % (max - min) + min;
    }
    println!("Generating a random prime number: {}", number);
}