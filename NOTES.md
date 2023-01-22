# Notes


## Explanation

This code implements the RC5 encryption algorithm. The algorithm is described in the [Rivest Cipher 5 (RC5) Specification](https://www.grc.com/r&d/rc5.pdf). The implementation is based on the [C reference implementation](https://www.grc.com/r&d/rc5.c) provided by Rivest.

I created a struct called `RC5` in ./src/rc5.rs which takes a key and a number of rounds as arguments. 
The struct has a generic type T which has to implement the trait `UInt` which is defined in ./src/uint.rs.
T is necessary to allow the algorithm to work with different word sizes. The algorithm only works with word sizes 8,16,32,64,128.
In order to support word sizes 80 and 24 I have implemented my own custom unsigned integer type `CustomUInt` in ./src/custom_uint.rs.
The `CustomUInt` struct is generic and can be used with any word size. The `CustomUInt` struct implements the `UInt` trait and therefore can be used as the generic type for the `RC5` struct.



## What I've learned

* implementing my own Custom Unsigned Integer type in Rust
* using a trait to implement arithmetic operations on my custom type and rusts built-in types
* how wrapping arithmetic works in Rust

## What I can improve

* The custom `Error` type could be improved using thiserror, but I didn't want to use any external crates for this project.
* The CustomUInt struct might be inefficient and should be tested on performance.
* Write cargo doc documentation for the code
* Currently pw and qw are hardcoded and should be calculated from the word_size, I implemented this calculation in `./src/rc5.rs:220` but it only works for word_size 8,16,32. I couldn't figure out how to implement this for word_size 64 and 128, without using external crates.
* The algorithm only works with word_size 8,16,32,64,128. Word Size 80 and 24 are failing the tests, from the logs i can see that they are failing in the setup function in './src/rc5.rs'. I couldn't figure out why this is happening.

