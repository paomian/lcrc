extern crate lcrc;
extern crate rustc_serialize;

use lcrc::object::common;
use rustc_serialize::json::Json;


#[test]
/*
pub fn test() {
    let mut obj = query_by_id(String::from("55fbbeb360b2780e16f6d30b"),
                          String::from("Post"));
    obj._descript = String::from("hello");
    println!("{:?}",obj);
}

pub fn query_by_id(id: String, class: String) -> LcObject {
    let data = get(&id,&class);
    let json = Json::from_str(&data).unwrap();
    let mut obj = LcObject::new(&class);
    println!("{}",obj.get_class());
    obj._descript = "Hello World";

}
*/

pub fn hello() {
    let mut x = common::LcObject::new("me");
    let mut y = common::LcObject::new("me");
    let mut z = common::LcObject::new("me");
    x.set("hello".to_string(),"world");
    x.set("paomian".to_string(),"ipaomian");
    me.save();
    y.set("x");
    println!("{}",me.object_id().unwrap());
    println!("{}",me.get("hello").unwrap());
    println!("{}",me.to_string().unwrap());
}
