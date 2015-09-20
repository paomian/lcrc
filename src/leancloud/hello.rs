use std::io::Read;

use hyper::Client;
use hyper::header::Connection;

pub fn hello() {
    let client = Client::new();

    let mut res = client.get("https://baidu.com")
        .header(Connection::close())
        .send().unwrap();

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    println!("Response: {}", body);
}
