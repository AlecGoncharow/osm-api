extern crate iron;
extern crate router;
extern crate mongodb;
extern crate serde;
extern crate serde_json;

use mongodb::{Client, ThreadedClient};
use std::env;
use iron::{status, Iron, IronResult, Request, Response};
use router::Router;
use std::fs::File;
use std::io::prelude::*;

mod osm;


// Serves a string to the user.  Try accessing "/".
fn hello(_: &mut Request) -> IronResult<Response> {
    let resp = Response::with((status::Ok, "Hello world!"));

    Ok(resp)
}

// simple route to allow for quick prototyping
fn campus_response(_: &mut Request) -> IronResult<Response> {
    let mut file = match File::open("static/campus.json") {
        Ok(f) => f,
        Err(e) => panic!("{:?}", e)
    };
    let mut contents = String::new();
    file.read_to_string(&mut contents).ok();
    let mut resp = Response::new();
    resp.body = Some(std::boxed::Box::new(contents));
    resp.status = Some(status::Ok);
    resp.headers.set(iron::headers::ContentType::json());

    Ok(resp)
}

// simple route to allow for quick prototyping
fn charlotte_response(_: &mut Request) -> IronResult<Response> {
    let mut file = match File::open("static/charlotte_nc.json") {
        Ok(f) => f,
        Err(e) => panic!("{:?}", e)
    };
    let mut contents = String::new();
    file.read_to_string(&mut contents).ok();
    let mut resp = Response::new();
    resp.body = Some(std::boxed::Box::new(contents));
    resp.status = Some(status::Ok);
    resp.headers.set(iron::headers::ContentType::json());

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
    router.get("/name/uncc_campus", campus_response, "uncc_campus");
    router.get("/name/charlotte", charlotte_response, "charlotte_nc");


    let client = Client::connect(&get_db_host(), get_db_port())
            .expect("Failed to initialize standalone client");

    router.get(
        "/mongo/name/:name",
        move |request: &mut Request| osm::handle_request(request, &client),
        "osm_name"
    );
    

    Iron::new(router)
        .http(("0.0.0.0", get_server_port()))
        .unwrap();
}
