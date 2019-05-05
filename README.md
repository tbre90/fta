# fta
fta (file to array) is a command line program that takes as its input any file, and produces a [C](https://en.wikipedia.org/wiki/C_(programming_language)) header file.

## Building and running
Download Rust: [https://www.rust-lang.org/learn/get-started](https://www.rust-lang.org/learn/get-started).

Clone the repository
```
git clone https://github.com/tbre90/fta
```
Build
```
cargo build
```
Optional: run tests
```
cargo test
```
Run
```
cargo run -- <name of input file>
```
This produces a C header file in the current working directory, whose name currently defaults to \<name of input file\>.h.

## Installing
Build a release binary
```
cargo build --release
```
Then you can grab the binary from the /target/release folder, and put it wherever you want it.

## License
This project is licensed under the Unlicense, see [https://unlicense.org/](https://unlicense.org/) or [LICENSE](https://github.com/tbre90/fta/blob/master/LICENSE).
