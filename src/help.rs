//Print help

pub fn print_help_short() {
    println!("Usage: (1) primeutils INTEGER [OPTIONS]");
    println!("       (2) primeutils --gen [OPTIONS]");
    println!("Type 'primeutils -h' for more help");
}

pub fn print_help() {
    println!("Usage: (1) primeutils INTEGER [OPTIONS] - Prime checking and factoring");
    println!("       (2) primeutils --gen [OPTIONS]   - Prime generation");
    println!("");
    println!("       (1) OPTIONS can be the following:");
    println!("           -c    Check INTEGER for compositeness");
    println!("           -a    Print the closest prime above INTEGER");
    println!("           -b    Print the closest prime below INTEGER");
    println!("           -f    Print the prime factorization of INTEGER");
    println!("           -e    Print everything");
    println!("           -h    Display this help screen");
    println!("       (2) OPTIONS can be the following:");
    println!("           --min=INT Set the lower limit to use");
    println!("           --max=INT Set the upper limit to use");
    println!("");
    println!("If options are omitted in (1), -c is automatically assumed");
    println!("If options are omitted in (2), the limits will be set");
    println!("to 2 and the upper unsigned int limit of the architecture");
    println!("");
    println!("The two usage options are mutually exclusive. With both");
    println!("specified, only the prime generating will be executed");
}
