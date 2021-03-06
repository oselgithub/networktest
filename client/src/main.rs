extern crate hyper;

use hyper::Client;
use std::io::Read;

fn main() {
    let client = Client::new();
    let mut res = client.get("http://localhost:3000/author/get_all").send().unwrap();
    assert_eq!(res.status, hyper::Ok);
    let mut s = String::new();
    res.read_to_string(&mut s).unwrap();
    println!("{}", s);
}
