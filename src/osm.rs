use mongodb::{Bson, bson, doc, Client, ThreadedClient, db::ThreadedDatabase};
use iron::{status, Iron, IronResult, Request, Response};
use router::Router;


pub fn handle_request(request: &mut Request, client: &HttpStream) -> IronResult<Response> {
    let params = req.extensions.get::<Router>().expect("Error getting router");
    let name = params.find("name").expect("missing parameter in router");


    let resp = Response::new();
    Ok(resp)
}