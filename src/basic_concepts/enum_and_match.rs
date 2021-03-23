use std::collections::HashMap;
use std::fmt::Display;

#[derive(Display, PartialEq, Eq, Hash)]
enum ProgFamilies {
    Procedural,
    OO,
    Logical,
    Functional,
}

pub fn enums() -> Option<String> {
    let mut my_map = HashMap::new();
    let mut c = String::new();
    c.push_str("C");
    my_map.insert(ProgFamilies::Procedural, c);

    let mut java: String = String::new();
    java.push_str("java");
    my_map.insert(ProgFamilies::OO, java);

    my_map.insert(ProgFamilies::Logical, String::from("prolog"));

    let clj: String = String::from("clojure");
    my_map.insert(ProgFamilies::Functional, clj);

    //let value: Option<&String> = my_map.get(&ProgFamilies::Procedural);
    for (k, v) in &my_map {
        println!("key: {}. val: {}.", k, v);
    }
    Some(String::from("MEOW"))
}
