#![allow(dead_code)]
mod pals;

extern crate crypto;
extern crate rand;
extern crate hyper;
extern crate url;

use url::{Url, UrlParser};

use hyper::Server;
use hyper::server::Request;
use hyper::server::Response;
use hyper::uri::RequestUri::AbsolutePath;

fn get_parts(path: &str) -> (String, String) {
    let base = Url::parse("http://localhost:9000").unwrap();
    let relative = UrlParser::new().base_url(&base).parse(path).unwrap();

    let mut file = String::new();
    let mut signature = String::new();

    for (key, value) in relative.query_pairs().unwrap() {
        match &*key {
            "file" => file = value,
            "signature" => signature = value,
            _ => {}
        };
    }

    (file, signature)
}

fn slow_equal(a: &[u8], b: &[u8]) -> bool {
    let min = std::cmp::min(a.len(), b.len());
    let time = std::time::Duration::from_millis(50);

    for i in 0..min {
        std::thread::sleep(time);

        if a[i] != b[i] {
            return false;
        }
    }

    return true;
}

fn hello(req: Request, mut res: Response) {
    let key = b"hodor";

    if let AbsolutePath(path) = req.uri {
        let (file, signature) = get_parts(&path);

        let computed = pals::sha1::hmac(key, file.as_bytes());
        let provided = pals::hex::decode(&signature).unwrap();

        if slow_equal(&computed, &provided) {
            *res.status_mut() = hyper::status::StatusCode::Ok;
            res.send(b"ok").unwrap();
        } else {
            *res.status_mut() = hyper::status::StatusCode::InternalServerError;
            res.send(b"invalid").unwrap();
        }
    } else {
        panic!("not expected");
    }
}

fn main() {
    Server::http("127.0.0.1:9000").unwrap().handle(hello).unwrap();
}
