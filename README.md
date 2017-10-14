# primeutils

A collection of prime number related utilities in a single command-line executable written in Rust.

```
Usage: primeutils INTEGER [OPTIONS]
       OPTIONS can be the following:
           -c    Check INTEGER for compositeness
           -n    Print the closest prime above INTEGER
           -p    Print the closest prime below INTEGER
           -f    Print the prime factorization of INTEGER
           -a    Print everything
           -h    Display this help screen
       If options are omitted, -c is automatically assumed
```

The upper range of the accepted numbers is limited by the architecture of the user, 2<sup>32</sup>-1 for 32-bit architectures, and 2<sup>64</sup>-1 for 64-bit ones.


