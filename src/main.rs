use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use std::io;

/// https://url.spec.whatwg.org/#fragment-percent-encode-set
const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

fn main() {
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Read Error");
    let out = utf8_percent_encode(&mut x.trim_end(), FRAGMENT);
    println!("{}", out);
}
