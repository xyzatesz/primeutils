//Print help

pub fn print_help() {
    println!("Usage: primeutils INTEGER [OPTIONS]");
    println!("       OPTIONS can be the following:");
    println!("           -c    Check INTEGER for compositeness");
    println!("           -a    Print the closest prime above INTEGER");
    println!("           -b    Print the closest prime below INTEGER");
    println!("           -f    Print the prime factorization of INTEGER");
    println!("           -e    Print everything");
    println!("           -h    Display this help screen");
    println!("           --gen Generate a random prime within 2 and 2^32-1");
    println!("");
    println!("If options are omitted, -c is automatically assumed");
    println!("Shorthand options and --gen are mutually exclusive.");
    println!("With both specified, only --gen will be executed");
}
