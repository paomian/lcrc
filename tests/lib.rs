extern crate lcrc;

use lcrc::object::query::query_by_id;


#[test]
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
