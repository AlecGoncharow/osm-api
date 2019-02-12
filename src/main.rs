extern crate iron;
extern crate router;

use std::env;
use iron::{status, Iron, IronResult, Request, Response};
use router::Router;

// Serves a string to the user.  Try accessing "/".
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

fn main() {
    let mut router: Router = Router::new();
    router.get("/", hello, "index");

    Iron::new(router)
        .http(("0.0.0.0", get_server_port()))
        .unwrap();
}
