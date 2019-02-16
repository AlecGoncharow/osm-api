use mongodb::{bson, doc, Client, ThreadedClient, db::ThreadedDatabase, db::Database};
use iron::{status, IronResult, Request, Response};
use router::Router;
use std::env;

fn get_db(client: &Client) -> Database {
    let name = get_db_name();
    let db = client.db(&name);
    if name != "test" {
        db.auth(&get_db_user(), &get_db_pass()).expect("error logging into to database");
    }
    db
}

fn get_db_name() -> String {
    env::var("MONGODB_NAME")
        .ok()
        .unwrap_or(String::from("test"))
}

fn get_db_user() -> String {
    env::var("MLAB_USER")
        .ok()
        .unwrap_or(String::from("test"))
}

fn get_db_pass() -> String {
    env::var("MLAB_PW")
        .ok()
        .unwrap_or(String::from("test"))
}

pub fn handle_request(request: &mut Request, client: &Client) -> IronResult<Response> {
    let params = request.extensions.get::<Router>().expect("Error getting router");
    let name = params.find("name").expect("missing parameter in router");
    let cities = get_db(client).collection("cities");

    let doc = doc! {
        "meta.name": name
    };

    let mut cursor = cities.find(Some(doc), None).ok().expect("Failed to execute find");
    let item = cursor.next();
    let mut resp = Response::new();
    match item {
        Some(Ok(doc)) => {
            let json_value = serde_json::to_value(&doc).expect("error casting bson into value");
            resp.body = Some(Box::new(json_value.to_string()));
            resp.status = Some(status::Ok);
            resp.headers.set(iron::headers::ContentType::json());
        }
        Some(Err(_)) => {
            resp.body = Some(Box::new("server error"));
            resp.status = Some(status::InternalServerError);
        },
        None => {
            resp.body = Some(Box::new("404 not found"));
            resp.status = Some(status::NotFound);
        }
    }

    Ok(resp)
}