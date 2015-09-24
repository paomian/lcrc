extern crate lcrc;

use lcrc::object::query::query_by_id;


#[test]
pub fn test() {
    let mut obj = query_by_id(String::from("55fbbeb360b2780e16f6d30b"),
                          String::from("Post"));
    obj._descript = String::from("hello");
    println!("{:?}",obj);
}
