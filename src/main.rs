use std::env;
use std::process;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let file = File::open(config.file_path).expect("couldn't open file");
    let reader = BufReader::new(file);

    //Sequences (in this case) are strings of length 90 containing only A,C,G and T.
    //Cell barcodes are strings of length 16 containing only A,C,G and T with the prefix "CB:Z:" and suffix "-1"
    let re_seq = Regex::new(r"[ACGT]{90}").unwrap();
    let re_cbc = Regex::new(r"CB:Z:[ACGT]{16}-1").unwrap();

    for line in reader.lines().filter_map(|x| x.ok()) {

        let seq = match re_seq.captures(&line) {
            None => "NA",
            Some(s) => s.get(0).map(|x| x.as_str()).unwrap()
        };

        let cbc = match re_cbc.captures(&line) {
            None => "NA",
            Some(c) => c.get(0).map(|x| x.as_str()).unwrap()
        };
        println!("{},{}", cbc,seq);
    }
}

struct Config {
    file_path: String
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let file_path = args[1].clone();

        Ok(Config {file_path})
    }
}