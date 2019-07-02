
extern crate serde;
extern crate serde_json;

use serde::Serialize;
use serde::Deserialize;
use serde_json::Result;



#[derive(Serialize, Deserialize)]
pub struct Person {
    pub name: String,
    pub age: u8,
    #[serde(rename = "iphone")]
    phones: Vec<String>,
}

pub enum Struct<T> {
    None,
    S(T),
}

pub fn json_to_struct<'a,T>(data: &'a str) -> Struct<T> where T: serde::Deserialize<'a> {
    let p = match serde_json::from_str(data) {
        Ok(p) => p,
        Err(_) => {return Struct::None},
    };
    return Struct::S(p);
}

pub fn struct_to_json_string<T>(s:&T) -> String where T: serde::Serialize {
    let js = match serde_json::to_string(&s) {
        Ok(js) => js,
        Err(_) => "".to_string(),
    };
    return js;
}

pub fn test_json() -> Result<()>  {
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "iphone": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    let p: Person = match json_to_struct(&data) {
        Struct::S(p) => p,
        Struct::None => panic!(),
    };
    println!("{:?}",p.age);
    let j = struct_to_json_string(&p);
    println!("{}", j);
    Ok(())
}
