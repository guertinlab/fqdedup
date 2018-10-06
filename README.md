# fqdedup

This [Rust](https://www.rust-lang.org) program removes PCR duplicates from FASTQ files. The driving goal was to process large files on a modest computer, so it trades some speed for less memory consumption by doing two things: 
1. encode the already seen sequences as a byte sequences (encoding 3 bases per byte, followed by a byte encoding the original sequence length); 
2. storing these byte sequences in a Patricia Tree (see https://en.wikipedia.org/wiki/Radix_tree and https://docs.rs/patricia_tree/0.1.1/patricia_tree/), which reduces the memory used by taking advantage of common prefixes.

## Usage

```bash
Usage:
    fqdedup [options] -i <filename>
    fqdedup (--help | --version)

Options:
  -h --help              Show this screen.
  --version              Show version.
  -o --output OUTPUT     Output filename (defaults to appending '_deduplicated' to the input name).
```
