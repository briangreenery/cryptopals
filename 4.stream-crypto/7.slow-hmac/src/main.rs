#![allow(dead_code)]
mod pals;

extern crate crypto;
extern crate rand;
extern crate hyper;

use hyper::Server;
use hyper::server::Request;
use hyper::server::Response;

fn hello(_: Request, res: Response) {
    res.send(b"Hello World!").unwrap();
}

fn main() {
    // println!("{}", pals::hex::encode(&pals::sha1::hmac(b"key", b"hodor")));
    Server::http("127.0.0.1:3000").unwrap().handle(hello).unwrap();
}
