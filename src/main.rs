extern crate mailparse;

use std::env;
use std::fs::File;
use std::io::prelude::*;

fn dump(pfx: &str, pm: &mailparse::ParsedMail) {
    println!(">> Headers from {} <<", pfx);
    for h in &pm.headers {
        println!("  [{}] => [{}]", h.get_key().unwrap(), h.get_value().unwrap());
    }
    println!(">> Body from {} <<", pfx);
    println!("  [{}]", pm.get_body().unwrap());
    let mut c = 1;
    for s in &pm.subparts {
        println!(">> Subpart {} <<", c);
        dump("subpart", s);
        c = c + 1;
    }
}

fn main() {
    let mut args = env::args();
    args.next();
    loop {
        match args.next() {
            None => break,
            Some(a) => {
                let mut f = File::open(&a).unwrap();
                let mut d = Vec::<u8>::new();
                f.read_to_end(&mut d).unwrap();
                let mail = mailparse::parse_mail(&d).unwrap();
                dump(&a, &mail);
            }
        }
    }
}
