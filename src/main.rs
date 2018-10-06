#[macro_use]
extern crate serde_derive;
extern crate docopt;
extern crate patricia_tree;

use docopt::Docopt;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::io::Lines;
use std::error::Error;
use patricia_tree::PatriciaSet;

const USAGE: &'static str = "
FastQ Deduplication

Usage:
    fqdedup [options] -i <filename>
    fqdedup (--help | --version)

Options:
  -h --help              Show this screen.
  --version              Show version.
  -o --output OUTPUT     Output filename (defaults to appending '_deduplicated' to the input name).
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_output: Option<String>,
    arg_filename: String,
}

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn get_line<B: BufRead>(iter: &mut Lines<B>) -> Option<String> {
    match iter.next() {
        Some(Ok(line)) => Some(line),
        Some(Err(err)) => {
            panic!(format!("Error: {}", err.description()));
        },
        None => None,
    }
}

#[inline]
fn base_index(base: u8) -> u8 {
    match base {
        b'a' | b'A' => 0,
        b'c' | b'C' => 1,
        b'g' | b'G' => 2,
        b't' | b'T' => 3,
        b'n' | b'N' => 4,
        _ => panic!(format!("Unknown base: {}", base as char))
    }
}

fn byte_pack_3_ext(seq: &str, res: &mut Vec<u8>) {
    res.clear();
    res.extend(seq.as_bytes()
           .chunks(3)
           .map(|chunk|
               chunk.iter()
                    .fold(0, |acc, &x| acc * 5 + base_index(x))
    ));
    res.push(seq.len() as u8);
}

fn output_filename(input: &str, opt: Option<String>) -> String {
    if let Some(name) = opt {
        name
    } else {
        if let Some(idx) = input.rfind('.') {
            let (basename, _) = input.split_at(idx);
            format!("{}_deduplicated.fastq", basename)
        } else {
            format!("{}_deduplicated.fastq", input)
        }
    }
}


fn dedup<R: BufRead, W: Write>(buf_reader: R, mut output: W) -> (usize, usize) {
    // processing
    let mut lines = buf_reader.lines();
    let mut count = 0;
    let mut filter_count = 0;
    let mut lookup = PatriciaSet::new();
    let mut key = Vec::new();

    loop {
        let header = get_line(&mut lines);
        let seq = get_line(&mut lines);
        let strand = get_line(&mut lines);
        let qual = get_line(&mut lines);

        if header.is_none() || seq.is_none() || strand.is_none() || qual.is_none() {
            break;
        } else {
            count = count + 1;
            // check if sequence is in lookup table
            byte_pack_3_ext(seq.as_ref().unwrap(), &mut key);
            let seen = lookup.contains(&key);

            if !seen {
                // if not, output the four lines
                lookup.insert(&key);
                filter_count = filter_count + 1;

                output.write_fmt(format_args!("{}\n", header.unwrap())).unwrap();
                output.write_fmt(format_args!("{}\n", seq.unwrap())).unwrap();
                output.write_fmt(format_args!("{}\n", strand.unwrap())).unwrap();
                output.write_fmt(format_args!("{}\n", qual.unwrap())).unwrap();
            }
        }
    }

    return (count, filter_count);
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| Ok(d.version(Some(format!("fqdedup v{}", VERSION)))))
                            .and_then(|d| d.deserialize())
                            .unwrap_or_else(|e| e.exit());

    // combinations of input and output
    let (count, filter_count) = 
        if args.flag_output.is_some() {
            if args.arg_filename == "-" && args.flag_output.as_ref().unwrap() == "-" {
                let stdin = std::io::stdin();
                let stdin_handle = stdin.lock();
                let stdout = std::io::stdout();
                let stdout_handle = stdout.lock();
                dedup(stdin_handle, stdout_handle)
            } else {
                let mut output = File::create(output_filename(&args.arg_filename, args.flag_output)).unwrap();
                if args.arg_filename == "-" {
                    let stdin = std::io::stdin();
                    let stdin_handle = stdin.lock();
                    dedup(stdin_handle, output)
                } else {
                    let file = File::open(&args.arg_filename).unwrap();
                    let buf_reader = BufReader::new(file);
                    dedup(buf_reader, output)
                }
            }
        } else {
            let mut output = File::create(output_filename(&args.arg_filename, args.flag_output)).unwrap();

            if args.arg_filename == "-" {
                let stdin = std::io::stdin();
                let stdin_handle = stdin.lock();
                dedup(stdin_handle, output)
            } else {
                let file = File::open(&args.arg_filename).unwrap();
                let buf_reader = BufReader::new(file);
                dedup(buf_reader, output)
            }
        };

    eprintln!("{} blocks!", count);
    eprintln!("{} selected!", filter_count);
}
