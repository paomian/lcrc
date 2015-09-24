use std::io::Read;

use hyper::Client;
//use hyper::header::Connection;
use hyper::header::{Headers, ContentType};
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use rustc_serialize::json::Json;

use super::config;
use super::common::LcObject;

header! { (XLcId, "X-LC-Id") => [String] }
header! { (XLcKey, "X-LC-Key") => [String] }

pub fn get(id: &String,class: &String) -> String {
    let client = Client::new();

    let mut headers = Headers::new();

    headers.set(
        ContentType(Mime(TopLevel::Application, SubLevel::Json,
                         vec![(Attr::Charset, Value::Utf8)])));
    headers.set(XLcId(config::APP_ID.to_string()));
    headers.set(XLcKey(config::APP_KEY.to_string()));


    let url = ["https://api.leancloud.cn/1.1/classes/",class.as_ref(),
               "/",id.as_ref()].concat();
    let mut res = client.get(&url).headers(headers).send().unwrap();

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    body
}

pub fn query_by_id(id: String, class: String) -> LcObject {
    let data = get(&id,&class);
    let json = Json::from_str(&data).unwrap();
    LcObject::new(&class,
                  "hello world",
                  json,)
}
