use std::io::Read;
use std::collections::HashMap;

use hyper::Client;
//use hyper::header::Connection;
use hyper::header::{Headers, ContentType};
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};

header! { (XLcId, "X-LC-Id") => [String] }
header! { (XLcKey, "X-LC-Key") => [String] }

enum LcValue {
    Bool (bool),
    StringType (String),
    Number (i64),
    Object (HashMap<String, LcValue>),
}

struct LcObject {
    _acl: LcValue,
    _data: LcValue,
    _class: String,
    _descript: String,
}

trait Construct {
    fn new(&self) -> LcObject;
}

/*
impl Construct for LcObject {
    fn new(&self) -> LcObject {
        //Todo
    }
}
 */

pub fn show_lc_value(arg: LcValue) {
    match arg {
        LcValue::Bool(x) => println!("{}",x),
        LcValue::StringType (x) => println!("{}",x),
        LcValue::Number(x) => println!("{}",x),
        //LcValue::Object(x) => println!("{}",x),
    }
}

pub fn hello() {
    let client = Client::new();

    let mut headers = Headers::new();

    headers.set(
        ContentType(Mime(TopLevel::Application, SubLevel::Json,
                         vec![(Attr::Charset, Value::Utf8)])));
    headers.set(XLcId("eASHChuf7eNHlvJHUtXXvNEA".to_owned()));
    headers.set(XLcKey("OizHjMgjVmTGGCCmkFcXMT6L".to_owned()));

    let mut ress = client.get("https://api.leancloud.cn/1.1/classes/Post/55fbbeb360b2780e16f6d30b")
        .headers(headers).send().unwrap();
    /*
    let mut res = client.get("https://leancloud.cn")
        .header(Connection::close())
        .send().unwrap();
    */

    let mut body = String::new();
    ress.read_to_string(&mut body).unwrap();

    println!("Response: {}", body);
}
