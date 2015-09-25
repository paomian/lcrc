use std::io::Read;


use rustc_serialize::json::Json;


use super::common::LcObject;



/*
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
*/
