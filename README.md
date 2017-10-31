# primeutils

A collection of prime number related utilities in a single command-line executable written in Rust.

```
Usage: (1) primeutils INTEGER [OPTIONS] - Prime checking and factoring
       (2) primeutils --gen [OPTIONS]   - Prime generation
    
       (1) OPTIONS can be the following:
              -c    Check INTEGER for compositeness
              -a    Print the closest prime above INTEGER
              -b    Print the closest prime below INTEGER
              -f    Print the prime factorization of INTEGER
              -e    Print everything
              -h    Display this help screen
       (2) OPTIONS can be the following:
              --min=INT Set the lower limit to use
              --max=INT Set the upper limit to use
    
If options are omitted in (1), -c is automatically assumed
If options are omitted in (2), the limits will be set
to 2 and the upper unsigned int limit of the architecture

The two usage options are mutually exclusive. With both
specified, only the prime generating will be executed
```

The upper range of the accepted numbers is limited by the architecture of the user, 2<sup>32</sup>-1 for 32-bit architectures, and 2<sup>64</sup>-1 for 64-bit ones.

## Building

For building, you need to have Rust and Cargo installed. After that, you can run `cargo build --release` to build for your current platform, or run `./autobuild.sh`, which will try to build it for both 32-bit and 64-bit versions of both Windows and Linux. For the script to work, you need to set up separate compilers for all four versions, and you need the `strip` GNU Coreutils command, which decreases the binary sizes significantly.

Alternatively, you can find prebuilt binaries under the 'Releases' tab for those four platforms.