# Vanith Ethereum Wallet Generator

This Rust project generates Ethereum addresses that match a specified prefix or suffix, also known as "vanity" addresses. The generation process can be parallelized across multiple threads to improve performance.

## Usage
To use the vanity Ethereum wallet generator, run the following command:

~~~shell
cargo run --release [options]
~~~

threads=<num> or t=<num>: Number of threads to use for generating wallets (default: 1)

number=<num> or n=<num>: Number of wallets to generate (default: 1)

prefix=<hex_string> or p=<hex_string>: Hex string to use as prefix for the wallet address.
~~~shell
suffix=<hex_string> or s=<hex_string>~~~: Hex string to use as suffix for the wallet address.

anywhere=<hex_string> or a=<hex_string>: Hex string to search for anywhere in the wallet address.

Note: Only one of prefix, suffix, or anywhere can be specified at a time.

## Examples
### Generate a single wallet with the prefix abc using 4 threads:

~~~shell
cargo run --release prefix=abc threads=4
~~~

or

~~~shell
cargo run --release p=abc t=4
~~~

### Generate 5 wallets with the suffix def using 2 threads:

~~~shell
cargo run --release suffix=def threads=2 number=5
~~~

or

~~~shell
cargo run --release s=def t=2 n=5
~~~
