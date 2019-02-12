extern crate iron;
extern crate router;

use std::env;
use iron::{status, Iron, IronResult, Request, Response};
use router::Router;
use std::fs::File;
use std::io::prelude::*;


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
    file.read_to_string(&mut contents);
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

fn main() {
    let mut router: Router = Router::new();
    router.get("/", hello, "index");
    router.get("/uncc_campus", campus_response, "uncc_campus");


    Iron::new(router)
        .http(("0.0.0.0", get_server_port()))
        .unwrap();
}
