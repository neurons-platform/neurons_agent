
extern crate reqwest;
use reqwest::header::{HeaderMap, HeaderValue,CONTENT_TYPE};
use std::collections::HashMap;
use super::time;
use super::json;



pub fn http_get(url:&str) -> String {
    let resp = reqwest::get(url);
    let mut r = match resp {
        Ok(res) => res,
        Err(_) => return "".to_string(),
    };
    let body = r.text();
    let result = match body {
        Ok(result) => result,
        Err(_) => return "".to_string(),
    };
    return result.to_string();
}


// pub fn http_post_json_with_header(url:String,data:String,headers:HeaderMap) -> String {
//     let client = reqwest::Client::new();
//     let mut res = match client.post(&url)
//         .headers(headers)
//         .json(&data)
//         .send() {
//             Ok(res) => res,
//             Err(err) => {
//                 println!("{}",err);
//                 return "".to_string();
//             },
//         };

//     let body = res.text();
//     let result = match body {
//         Ok(result) => result,
//         Err(err) => {
//             println!("{}",err);
//             return "".to_string();
//         },
//     };
//     return result.to_string();

// }

pub fn http_post_json_with_header<T>(url:String,data:&T,headers:HeaderMap) -> String
where T: serde::Serialize {
    let client = reqwest::Client::new();
    let mut res = match client.post(&url)
        .headers(headers)
        .json(data)
        .send() {
            Ok(res) => res,
            Err(err) => {
                println!("{}",err);
                return "".to_string();
            },
        };

    let body = res.text();
    let result = match body {
        Ok(result) => result,
        Err(err) => {
            println!("{}",err);
            return "".to_string();
        },
    };
    return result.to_string();

}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_http_post_json_with_header() {
        // let data = "{\"a\":1}";
        // let p = json::Person {age:12,name:"jimila".to_string()};
        // let j = json::struct_to_json_string(&p);
        // http_post_json_with_header("http://127.0.0.1:5000/train".to_string(),j,get_mq_header("123"));
    }

}
