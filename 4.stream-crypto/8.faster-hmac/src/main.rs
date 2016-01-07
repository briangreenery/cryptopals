#![allow(dead_code)]
mod pals;

extern crate crypto;
extern crate rand;
extern crate hyper;
extern crate url;
extern crate time;

use url::{Url, UrlParser};

use hyper::Server;
use hyper::server::Request;
use hyper::server::Response;
use hyper::uri::RequestUri::AbsolutePath;

use hyper::Client;

use std::thread;

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

fn insecure_compare(a: &[u8], b: &[u8]) -> bool {
    let min = std::cmp::min(a.len(), b.len());

    for i in 0..min {
        if a[i] != b[i] {
            return false;
        }

        std::thread::sleep_ms(5);
    }

    return true;
}

fn hello(req: Request, mut res: Response) {
    let key = b"hodor";

    if let AbsolutePath(path) = req.uri {
        let (file, signature) = get_parts(&path);

        let computed = pals::sha1::hmac(key, file.as_bytes());
        let provided = pals::hex::decode(&signature).unwrap();

        if insecure_compare(&computed, &provided) {
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

fn try(file: &str, signature: &[u8; 20]) -> bool {
    let client = Client::new();

    let mut url = String::new();
    url.push_str("http://127.0.0.1:9000/test?file=");
    url.push_str(file);
    url.push_str("&signature=");
    url.push_str(&pals::hex::encode(signature));

    let res = client.get(&*url)
                    .header(hyper::header::Connection::close())
                    .send()
                    .unwrap();

    res.status == hyper::Ok
}

fn now() -> i64 {
    let timespec = time::now_utc().to_timespec();
    (1000 * timespec.sec) + (timespec.nsec as i64 / 1000 / 1000)
}

fn main() {
    let computed = pals::sha1::hmac(b"hodor", b"foo");

    let handle = thread::spawn(|| {
        Server::http("127.0.0.1:9000").unwrap().handle(hello).unwrap();
    });

    let mut buffer = [0; 20];
    let mut counter = 0;

    println!("waiting");
    std::thread::sleep_ms(60000);

    for index in 0..buffer.len() {
        println!("computed: {}", &pals::hex::encode(&computed));
        println!("guessed:  {}", &pals::hex::encode(&buffer));

        let mut best_time = 0;
        let mut best_byte = 0;

        for byte in 0..256 {
            buffer[index] = byte as u8;

            let mut trials = Vec::new();

            let runs = 16;
            let discard = 8;

            for _ in 0..runs {
                counter += 1;

                let start = now();
                try("foo", &buffer);
                trials.push(now() - start);
            }

            trials.sort();

            let sum = trials.iter().take(runs - discard).fold(0, |a, b| a + b);

            if sum > best_time {
                best_time = sum;
                best_byte = byte as u8;

                println!("best is {} at {}",
                         &pals::hex::encode(&[best_byte]),
                         best_time);
            }

            if counter > 10000 {
                println!("waiting");
                counter = 0;
                std::thread::sleep_ms(60000);
            }
        }

        buffer[index] = best_byte;
    }

    if try("foo", &buffer) {
        println!("valid signature is {}", &pals::hex::encode(&buffer));
    }

    handle.join().unwrap();
}
