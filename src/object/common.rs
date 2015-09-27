use rustc_serialize::json::Json;
use rustc_serialize::json::Json::Object;
use std::option::Option;
use std::collections::BTreeMap;

use hyper::Client;
//use hyper::header::Connection;
use hyper::header::{Headers, ContentType};
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};

header! { (XLcId, "X-LC-Id") => [String] }
header! { (XLcKey, "X-LC-Key") => [String] }

use super::config;

#[derive(Debug)]
pub struct LcObject {
    pub _data: Option<BTreeMap<String,Json>>,
    _class: String,
    pub _descript: Option<String>,
    _be_saved: bool,
}

impl LcObject {
    pub fn new(class: &str) -> LcObject {
        LcObject {_data: None,
                  _class: String::from(class),
                  _descript: None,
                  _be_saved: false}
    }

    pub fn get_class(&self) -> String {
        self._class.clone()
    }

    pub fn saved(&self) -> bool {
        self._be_saved
    }

    pub fn set(&mut self, key: String, value: Json) {
        if let Some(ref mut data) = self._data {
            data.insert(key,value);
        } else {
            let mut map = BTreeMap::new();
            map.insert(key,value);
            self._data = Some(map);
        }
    }

    pub fn get(&self,key: String) -> Option<Json> {
        if let Some(ref data) = self._data {
            Some(data.get(&key).unwrap().clone())
        } else {
            None
        }
    }

    pub fn remove (&mut self, key: String) -> Option<Json> {
        if let Some(ref mut data) = self._data {
            data.remove(&key)
        } else {
            None
        }
    }

    pub fn to_string(&self) -> Option<String> {
        if let Some(ref data) = self._data {
            Some(Json::Object(data.to_owned()).to_string())
        } else {
            None
        }
    }

    pub fn from_string(&self) -> &Self
}

//static CLIENT: Client = Client::new();
/*
pub gen_url(id: Option<&String>, class: &String,url_parma: Option<&String>) -> String {

}

pub yong_bao(method:&String,json?:bool,
             url_parma: Option<&String>,id: Option<&String>,
             class: &String) -> LcObject {
    let mut headers = Headers::new();
    headers.set(XLcId(config::APP_ID.to_string()));
    headers.set(XLcKey(config::APP_KEY.to_string()));
    if match json? {
        Some(v) => v,
        None => false,
    } {
        headers.set(
        ContentType(Mime(TopLevel::Application, SubLevel::Json,
                         vec![(Attr::Charset, Value::Utf8)])));
    }

    let url = [API ,class.as_ref(),
               "/",id.as_ref()].concat();
}
*/
