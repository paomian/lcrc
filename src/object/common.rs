use rustc_serialize::json::Json;

#[derive(Debug)]
pub struct LcObject {
    pub _data: Json,
    _class: String,
    pub _descript: String,
}



impl LcObject {
    pub fn new(class: &str, descript: &str, data: Json) -> LcObject {
        LcObject {_data: data,
                  _class: String::from(class),
                  _descript: String::from(descript)}
    }
}
