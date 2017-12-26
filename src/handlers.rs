extern crate iron;
extern crate serde_json;

#[macro_use] 
extern crate serde_derive;

use iron::prelude::*;
use iron::{headers, status};
use iron::modifiers::Header;
use router::Router;
use persistent::{State, Read};

use todo::*;

pub fn get_version(_: &mut Request) -> IronResult<Response> {
    let version = env!("CARGO_PKG_VERSION");
    return Ok(Response::with((status::Ok, version)));
}

pub fn get_todo(r: &mut Request) -> IronResult<Response> {
    let id_option = r.extensions.get::<Router>().unwrap().find("id").parse::<u32>();

    match id_option {
        Ok(id) => {
            let mutex = r.get::<State<Storage<Todo>>>().unwrap();
            let storage = mutex.read().unwrap();
            match storage.get(&id) {
                Some(todo) => Ok(Response::with((status::Ok, 
                    Header(headers::ContentType::json()), 
                    serde_json::to_string(&todo).unwrap()))),
                None => Ok(Response::with((status::NotFound, format!("todo with ID {} was not found", id))))
            }
        }
        Err(e) => Ok(Response::with((status::BadRequest, format!("could not parse input: {}", e.description()))))
    }
}