extern crate mailparse;

use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut args = env::args();
    args.next();
    loop {
        match args.next() {
            None => break,
            Some(a) => {
                let mut f = File::open(a.clone()).unwrap();
                let mut d = Vec::<u8>::new();
                f.read_to_end(&mut d).unwrap();
                let (headers, _) = mailparse::parse_headers(&d).unwrap();
                println!(">> Headers from {} <<", a);
                for h in headers {
                    println!("  [{}] => [{}]", h.get_key().unwrap(), h.get_value().unwrap());
                }
            }
        }
    }
}
