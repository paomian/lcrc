use rustc_serialize::json::Json;
use rustc_serialize::json::Json::Object;

use std::option::Option;
use std::collections::BTreeMap;
use std::result::Result;

use std::io::Read;
use std::convert::From;
use std::borrow::Borrow;

use hyper;
use hyper::Client;
//use hyper::header::Connection;
use hyper::header::{Headers, ContentType};
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};

//use log::Log;

use super::config;

header! { (XLcId, "X-LC-Id") => [String] }
header! { (XLcKey, "X-LC-Key") => [String] }



pub fn save(json: &String,class: &String) -> Result<Json,String> {
    let mut headers = Headers::new();
    headers.set(XLcId(config::APP_ID.to_string()));
    headers.set(XLcKey(config::APP_KEY.to_string()));
    headers.set(
        ContentType(Mime(TopLevel::Application, SubLevel::Json,
                         vec![(Attr::Charset, Value::Utf8)])));
    let url = [config::API, "classes/", &class[..]].concat();
    let client = Client::new();

    let mut res = client.post(&url)
        .headers(headers)
        .body(json).send().unwrap();
    if res.status == hyper::status::StatusCode::Created {
        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        info!("{}",body);
        Ok(Json::from_str(&body).unwrap())
    } else {
        info!("{}",res.status);
        info!("{}",res.url);
        Err(String::from("error"))
    }
}

/*
pub enum Json {
    I64(i64),
    U64(u64),
    F64(f64),
    String(String),
    Boolean(bool),
    Array(Array),
    Object(Object),
    Null,
}
*/


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
    _objectid: Option<String>,
    pub _descript: Option<String>,
    _be_saved: bool,
}

pub trait IntoJson {
    fn into_json(&self) -> Json;
}

macro_rules! it_json {
    ($t:ty,$p:ident) => (
        impl IntoJson for $t {
            #[inline]
            fn into_json(&self) -> Json {
                Json::$p(self.clone())
            }
        }
        )
}


it_json!(i64,I64);
it_json!(u64,U64);
it_json!(f64,F64);
it_json!(String,String);
it_json!(bool,Boolean);

impl IntoJson for str {
    #[inline]
    fn into_json(&self) -> Json {
        Json::String(String::from(self))
    }
}

impl IntoJson for Json {
    #[inline]
    fn into_json(&self) -> Json {
        self.clone()
    }
}

impl IntoJson for LcObject{
    #[inline]
    fn into_json(&self) -> Json {
        if self._be_saved {
            let mut tmp = BTreeMap::new();
            tmp.insert("__type".to_string(),"Pointer".into_json());
            tmp.insert("className".to_string(),(&self._class).into_json());
            self._objectid.clone()
                .and_then(|id|
                          tmp.insert("objectId".to_string(),
                                     id.into_json()));
            Json::Object(tmp)
        } else {
            match self._data {
                Some(ref data) => Json::Object(data.clone()),
                None => Json::Object(BTreeMap::new()),
            }
        }
    }
}

impl LcObject {
    pub fn new(class: &str) -> LcObject {
        LcObject {_data: None,
                  _class: String::from(class),
                  _descript: None,
                  _be_saved: false,
                  _objectid: None}
    }

    pub fn get_class(&self) -> &String {
        &self._class
    }

    pub fn saved(&self) -> bool {
        self._be_saved
    }

    pub fn set<K: Into<String>,V: IntoJson>(&mut self, key: K, value: V) -> Option<Json> {
        let v = value.into_json();
        if let Some(ref mut data) = self._data {
            data.insert(key.into(),v)
        } else {
            let mut map = BTreeMap::new();
            map.insert(key.into(),v);
            self._data = Some(map);
            None
        }
    }

    pub fn get<K: Ord + ?Sized>(&mut self,key: &K) -> Option<&mut Json>
        where String: Borrow<K> {
            if let Some(ref mut data) = self._data {
                data.get_mut(key)
            } else {
                None
            }
        }

    pub fn remove<K: Ord + ?Sized>(&mut self, key: &K) -> Option<Json>
    where String: Borrow<K>{
        if let Some(ref mut data) = self._data {
            data.remove(key)
        } else {
            None
        }
    }

    pub fn object_id(&self) -> Option<String> {
        if self._be_saved {
            self._objectid.to_owned()
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

    pub fn save(&mut self) -> Result<bool,String> {
        if let Some(_) = self._data {
            let json = self.to_string().unwrap();
            if let Ok(ref data) = save(&json,&self._class) {
                let o = data.as_object();
                self._objectid = o.and_then(|x|x.get("objectId"))
                    .and_then(|x| x.as_string())
                    .map(|x| x.to_string());
                o.and_then(|x| x.get("createdAt"))
                    .map(|x| self.set("createdAt",x.clone()));
                self._be_saved = true;
                Ok(true)
            } else {
                Err(String::from("error"))
            }
        } else {
            Err(String::from("error"))
        }
    }
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
