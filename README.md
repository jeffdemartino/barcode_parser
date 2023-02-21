# Barcode parser

Preface: There are probably quite a few simpler ways of solving the problem this tool was meant to address, but my motivation was mostly to learn/practice coding in Rust

## Background

The purpose of this command line tool is to extract the mapped sequences and corresponding 10X genomics cell barcodes from a SAM file containing reads of interest (those which map to our lineage tracing barcode construct). Previously, this task was performed in R by reading in the entire SAM file, splitting each line using `str_split` (from the stringr package) and extracting the fields of interest. This took a massive amount of time (>30 minutes) for files containing a large amount of reads (>80,000) and consumed a huge amount of memory. This implementation circumvents these problems by reading an input SAM file line by line, finding the fields using regular expressions (implemented by the [regex crate](https://crates.io/crates/regex)) and dumping them into `stdout`. If the program can't find either the sequence or cell barcode it will substitute an `NA` value. The same file of >80,000 reads takes only a few seconds to parse and uses a very small amount of memory. 

Note, the regular expressions are very specific and will return `NA` if the read lengths are not exactly 90 base pairs. Additionally, there are no checks to ensure the input is a valid SAM file.

## Installation

To run this program, you first need to have a stable version of the Rust language installed. The easiest way to do this is using [rustup](https://rustup.rs/), which will also install the Rust package manager (Cargo)[https://doc.rust-lang.org/cargo/index.html]

Once you have Cargo installed you can clone this repository and compile the program locally:
```
git clone https://github.com/jeffdemartino/barcode_parser.git && cd barcode_parser
cargo build --release
```

## Usage
Assuming you compiled as described above, the binary file should be located in `./target/release`
The program can then run as follows, feeding an input sam file and saving the `stdout` to an output text file:
```
./barcode_parser INPUT.sam > output.txt
```

The output text file can then be read into R for further processing an analysis

## Improvements
There are a number of potential improvements which would make this program more useful:
- [ ] Enable filtering of, for example, reads which have either the sequence or cell barcode missing (not useful to have these in the output)
- [ ] Instead of returning the full 90bp sequence, pull the actual 20bp barcode out of the reads and return this (at the moment this is being done in R, but would be easy to implement with an additional regex)
- [ ] Choose the read length in case this varies