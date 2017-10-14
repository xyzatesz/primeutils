//Print help

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
