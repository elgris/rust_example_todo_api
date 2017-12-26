extern crate clap;
extern crate iron;
extern crate router;

use clap::App;
use iron::prelude::*;
use iron::status;
use router::Router;

fn get_version(_: &mut Request) -> IronResult<Response> {
    let version = env!("CARGO_PKG_VERSION");
    return Ok(Response::with((status::Ok, version)));
}

fn main() {
    let matches = App::new("example-todo-app")
                        .about("Example of a TODO API in Rust")
                        .args_from_usage("-l, --listen=[address:port] 'Sets an address and port to listen''")
                        .get_matches();
    let listen = matches.value_of("listen")
                        .unwrap_or("127.0.0.1:3000");
    let mut router = Router::new();
    router.get("/version", get_version, "version");
    Iron::new(router).http(listen).unwrap();
}