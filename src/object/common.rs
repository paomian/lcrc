use rustc_serialize::json::Json;
use rustc_serialize::json::Json::Object;

use std::option::Option;
use std::collections::BTreeMap;
use std::result::Result;

use hyper::Client;
//use hyper::header::Connection;
use hyper::header::{Headers, ContentType};
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};

use super::config;

header! { (XLcId, "X-LC-Id") => [String] }
header! { (XLcKey, "X-LC-Key") => [String] }

#[derive(Clone, Copy, PartialEq)]
pub enum LcObjectErrorCode {
    NotOneObject,
}

pub enum ObjectError {
    NotOneObjectError(LcObjectErrorCode,usize,usize),
    Other(String),
}


#[derive(Debug)]
pub struct LcObject {
    _data: Option<BTreeMap<String,Json>>,
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

    pub fn set(&mut self, key: String, value: Json) -> Option<Json> {
        if let Some(ref mut data) = self._data {
            data.insert(key,value)
        } else {
            let mut map = BTreeMap::new();
            map.insert(key,value);
            self._data = Some(map);
            None
        }
    }

    pub fn get(&self,key: &str) -> Option<&Json> {
        if let Some(ref data) = self._data {
            data.get(key)
        } else {
            None
        }
    }

    pub fn remove (&mut self, key: &str) -> Option<Json> {
        if let Some(ref mut data) = self._data {
            data.remove(key)
        } else {
            None
        }
    }

    pub fn object_id(&self) -> Option<String> {
        if let Some(ref data) = self._data {
            Some(data.get("objectId").unwrap().as_string().unwrap().to_owned())
        } else {
            None
        }
    }

    pub fn to_string(&self) -> Option<String> {
        if let Some(ref data) = self._data {
            Some(Json::Object(data.clone()).to_string())
        } else {
            None
        }
    }

    pub fn from_string(mut self, json: String) -> Result<Option<Self>,ObjectError> {
        let data = Json::from_str(&json).unwrap();
        if !data.is_object() {
            return Err(ObjectError::NotOneObjectError(LcObjectErrorCode::NotOneObject,0,0));
        }
        if let Some(ref mut self_data) = self._data {
            for (key, value) in data.as_object().unwrap().iter() {
                self_data.insert(key.to_owned(),value.to_owned());
            }
        } else {
            self._data = Some(data.as_object().unwrap().clone());
        }
        Ok(Some(self))
    }

    pub save(&mut self) -> Result<>
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
