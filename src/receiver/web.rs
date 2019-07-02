

extern crate iron;
extern crate router;
extern crate rustc_serialize;

use iron::prelude::*;
use iron::status;
use router::Router;
use rustc_serialize::json;
use std::io::Read;

#[derive(RustcEncodable, RustcDecodable)]
struct Greeting {
    msg: String
}

fn hello_world(_: &mut Request) -> IronResult<Response> {
    let greeting = Greeting { msg: "Hello, World".to_string() };
    let payload = json::encode(&greeting).unwrap();
    Ok(Response::with((status::Ok, payload)))
}

fn set_greeting(request: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();
    match request.body.read_to_string(&mut payload) {
        Ok(_) => {},
        Err(err) => {println!("{:?}",err)}
    };
    let request: Greeting = json::decode(&payload).unwrap();
    let greeting = Greeting { msg: request.msg };
    let payload = json::encode(&greeting).unwrap();
    Ok(Response::with((status::Ok, payload)))
}

pub fn start_server() {
    let mut router = Router::new();

    router.get("/", hello_world,"hello_world");
    router.post("/set", set_greeting,"set_greeting");
    Iron::new(router).http("localhost:3000").unwrap();
}
