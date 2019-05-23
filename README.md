# fqdedup

This [Rust](https://www.rust-lang.org) program removes PCR duplicates from FASTQ files. The driving goal was to process large files on a modest computer, so it trades some speed for less memory consumption by doing two things: 
1. encode the already seen sequences as a byte sequences (encoding 3 bases per byte, followed by a byte encoding the original sequence length); 
2. storing these byte sequences in a Patricia Tree (see https://en.wikipedia.org/wiki/Radix_tree and https://docs.rs/patricia_tree/0.1.1/patricia_tree/), which reduces the memory used by taking advantage of common prefixes.

## Install
fqdedup is written in Rust, so it requires Rust and Cargo to compile it (www.rust-lang.org). fqdedup is known to run on OS X and Linux. It may be possible to compile it on other platforms as long as supporting Rust libraries can be made to compile, specifically https://crates.io/crates/flate2 and https://crates.io/crates/rust-htslib are likely to be the dependencies that are most troublesome to compile.

To install Rust and Cargo visit www.rust-lang.org, fqdedup should compile with Rust 1.18.0 or later. Alternatively, on OS X, if you have Homebrew ( http://brew.sh/ ) installed and up to date, you can use brew to install rust as so:
```bash
brew install rust
```
Afterwards, uncompress the source and build:

```bash
tar xzf fqdedup-1.0.0.tar.gz
cd fqdedup-1.0.0
cargo build --release 
```

After compilation, copy the fqdedup binary (fqdedup-1.0.0/target/release/fqdedup) to a folder in your PATH, for example /usr/local/bin.

### Usage

```bash
Usage:
    fqdedup [options] -i <filename>
    fqdedup (--help | --version)

Options:
  -h --help              Show this screen.
  --version              Show version.
  -o --output OUTPUT     Output filename (defaults to appending '_deduplicated' to the input name).
```
