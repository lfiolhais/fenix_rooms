extern crate pencil;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate hyper;

use std::collections::BTreeMap;
use pencil::{Pencil, Request, Response, PencilResult};
use pencil::jsonify;
use hyper::client::Client;
use std::io::Read;

/// Creates a User in the Database
///
/// Create a user in the database with the specified username in the body. If
/// multiple usernames are provided only the first will be considered. If the no
/// username is provided an error message will be displayed with the status code
/// 204. Otherwise the username will be created and the JSON message will be
/// sent with the username and the ID given by the DB.
///
/// # Arguments
/// * request - The request sent by the client
///
/// # Output
/// A Response with a JSON messsage and correct status code.
fn create_user(request: &mut Request) -> PencilResult {

    // Get the username from the body of the request if it exists
    let username: &str = match request.form().get("Some") {
        Some(username) => username,
        None => "",
    };

    // Need to change the jsonify function to accept other status codes.
    let mut object = BTreeMap::new();
    object.insert("username", username);

    // Query DB about the availability of username
    // If username exists send a response with the 409 status code
    // else create user

    // let mut response: Response;
    // if username.is_empty() {
    //     response = Response::from("Username was not provided");
    //     response.status_code = 204;
    // } else {
    //     response = Response::from(username);
    //     response.status_code = 201;
    //     response.set_content_type("application/json");
    // }

    return jsonify(&object);
}

/// Get all campi at IST
///
/// # Output
/// All campi
fn get_campi(request: &mut Request) -> PencilResult {
    // Create Hyper client to perform REST calls
    let client = Client::new();

    // Create and send GET request
    let mut res =
        client.get("https://fenix.tecnico.ulisboa.pt/api/fenix/v1/spaces").send().unwrap();
    // Read content from response and write it to a buffer
    let mut buf: String = String::new();
    res.read_to_string(&mut buf);

    // Build response and set contetn to JSON
    let mut response = Response::from(buf);
    response.set_content_type("application/json");

    return Ok(response);
}

fn main() {
    let mut app = Pencil::new("~/fenix-rooms/src");
    app.set_debug(true);
    app.set_log_level();
    env_logger::init().unwrap();

    // ///////////////////////////////////////////////////////
    // Routing
    // ///////////////////////////////////////////////////////

    // //////////////////////////////////////////////////////
    // Templates
    // The standard browser routes through here.
    // ///////////////////////////////////////////////////////

    // ///////////////////////////////////////////////////////
    // REST API
    // The REST API will only return JSON enconded responses.
    // ///////////////////////////////////////////////////////

    // Admin
    // Admin always has user-id 0
    app.get("/api/user-id/0/campi", "admin", get_campi);

    // User
    // User can have any other user-id except 0
    app.post("/api/create_user", "create_user", create_user);

    // Run server
    debug!("Running on 127.0.0.1:8080");
    app.run("127.0.0.1:8080");
}
