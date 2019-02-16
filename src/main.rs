extern crate iron;
extern crate router;
extern crate mongodb;
extern crate serde;
extern crate serde_json;

use mongodb::{Client, ThreadedClient};
use std::env;
use iron::{status, Iron, IronResult, Request, Response};
use router::Router;

mod osm;

fn hello(_: &mut Request) -> IronResult<Response> {
    let resp = Response::with((status::Ok, "Hello world!"));

    Ok(resp)
}

/// Look up our server port number in PORT, for compatibility with Heroku.
fn get_server_port() -> u16 {
    env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080)
}

fn get_db_port() -> u16 {
    env::var("MLAB_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(27017)
}

fn get_db_host() -> String {
    env::var("MLAB_HOST")
        .ok()
        .unwrap_or(String::from("localhost"))
}

fn main() {
    let mut router: Router = Router::new();
    router.get("/", hello, "index");

    let client = Client::connect(&get_db_host(), get_db_port())
            .expect("Failed to initialize standalone client");

    router.get(
        "/name/:name",
        move |request: &mut Request| osm::handle_request(request, &client),
        "osm_name"
    );
    

    Iron::new(router)
        .http(("0.0.0.0", get_server_port()))
        .unwrap();
}
