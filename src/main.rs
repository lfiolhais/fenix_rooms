extern crate pencil;
#[macro_use]
extern crate log;
extern crate env_logger;

use std::collections::BTreeMap;
use pencil::{Pencil, Request, Response, PencilResult};
use pencil::jsonify;

fn create_user(request: &mut Request) -> PencilResult {

    // Get the username from the body of the request if it exists
    let username: &str = match request.form().get("username") {
        Some(username) => username,
        None => "",
    };

    // Need to change the jsonify function to accept other status codes.
    let mut object = BTreeMap::new();
    object.insert("username", username);

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

fn main() {
    let mut app = Pencil::new("~/fenix-rooms/src");
    app.set_debug(true);
    app.set_log_level();
    env_logger::init().unwrap();

    /// ///////////////////////////////////////////////////////
    /// Routing
    /// ///////////////////////////////////////////////////////

    /// ///////////////////////////////////////////////////////
    /// Templates
    /// The standard browser routes through here.
    /// ///////////////////////////////////////////////////////

    /// ///////////////////////////////////////////////////////
    /// REST API
    /// The REST API will only return JSON enconded responses.
    /// ///////////////////////////////////////////////////////

    // Admin
    // Admin always has user-id 0
    // app.get("/api/user-id/0", "admin", );

    // User
    // User can have any other user-id except 0
    app.post("/api/create_user", "create_user", create_user);

    // Run server
    debug!("Running on 127.0.0.1:8080");
    app.run("127.0.0.1:8080");
}
