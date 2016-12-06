#![feature(proc_macro)]

extern crate pencil;
extern crate fenix_rooms;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate hyper;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use std::collections::HashMap;
use pencil::{Pencil, Request, Response, PencilResult};
use pencil::jsonify;
use pencil::{UserError, PenUserError};
use hyper::client::Client;
use std::io::Read;
use fenix_rooms::utils::get_request;

#[derive(Deserialize)]
struct ContainedSpace {
    #[serde(rename="type")]
    type_name: String,
    id: String,
    name: String,
    #[serde(rename="topLevelSpace")]
    top_level_space: HashMap<String, String>,
}

#[derive(Deserialize)]
struct Building {
    #[serde(rename="type")]
    type_name: String,
    id: String,
    name: String,
    #[serde(rename="topLevelSpace")]
    top_level_space: HashMap<String, String>,
    #[serde(rename="containedSpaces")]
    contained_spaces: Vec<ContainedSpace>,
    #[serde(rename="parentSpace")]
    parent_space: HashMap<String, String>,
}

#[derive(Deserialize)]
struct Campus {
    #[serde(rename="type")]
    type_name: String,
    id: String,
    name: String,
    #[serde(rename="containedSpaces")]
    contained_spaces: Vec<ContainedSpace>,
}

type Space = Vec<HashMap<String, String>>;

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
// fn create_user(request: &mut Request) -> PencilResult {
//
//     // Get the username from the body of the request if it exists
//     let username: &str = match request.form().get("username") {
//         Some(username) => username,
//         None => "",
//     };
//
//     // Need to change the jsonify function to accept other status codes.
//     let mut object = BTreeMap::new();
//     object.insert("username", username);
//
//     // Query DB about the availability of username
//     // If username exists send a response with the 409 status code
//     // else create user
//
//     // let mut response: Response;
//     // if username.is_empty() {
//     //     response = Response::from("Username was not provided");
//     //     response.status_code = 204;
//     // } else {
//     //     response = Response::from(username);
//     //     response.status_code = 201;
//     //     response.set_content_type("application/json");
//     // }
//
//     return jsonify(&object);
// }

/// Get all campi at IST
///
/// Build a GET request and query FenixEDU. The response body will have a list
/// of dictionaries. Quietly bail if we fail to read the response body. If the
/// message read more than 0 bytes we can proceed to processing the information.
/// Otherwise bail quietly.
///
/// # Output
/// All campi information with type, name and id.
fn get_campi() -> Result<(Space, String), UserError>  {

    // Send GET request to the url
    let get_response = match get_request("https://fenix.tecnico.ulisboa.pt/api/fenix/v1/spaces") {
        Ok(res) => res,
        Err(err) => {
            let error = UserError::new(err);
            return Err(error);
        }
    };

    let space: Space = serde_json::from_str(&get_response).unwrap();

    println!("Space: {:#?}", space);

    return Ok((space, get_response));
}

fn campi_handler(_: &mut Request) -> PencilResult {
    let space = match get_campi() {
        Ok(space) => space.1,
        Err(err) => {
            return Err(PenUserError(err));
        }
    };

    // Build response and set content to JSON
    let mut response = Response::from(space);
    response.set_content_type("application/json");

    return Ok(response);
}

fn get_building(request: &mut Request) -> PencilResult {
    let campus = match request.view_args.get("campus") {
        Some(campus) => campus,
        None => "",
    };

    if campus.is_empty() {
        let error = UserError::new("The campus field is empty");
        return Err(PenUserError(error));
    } else {
        let space = match get_campi() {
            Ok(space) => space.0,
            Err(err) => {
                return Err(PenUserError(err));
            }
        };

        let mut fenix_campus_id: &String = &format!("");
        // This needs to work for the other campus "Tecnológico e Nuclear"
        for c in &space {
            if c.get("name").unwrap().to_lowercase() == campus {
                fenix_campus_id = c.get("id").unwrap();
                break;
            }
        }

        if fenix_campus_id.is_empty() {
            return Ok(Response::from(format!("There was no campus found with name: {}", campus)));
        }

        println!("The id found for {} is: {}", campus, fenix_campus_id);

        let url = &format!("https://fenix.tecnico.ulisboa.pt/api/fenix/v1/spaces/{}",
                           fenix_campus_id);

        let campus = match get_request(url) {
            Ok(campus) => campus,
            Err(err) => {
                let error = UserError::new(err);
                return Err(PenUserError(error));
            }
        };

        let building: Campus = serde_json::from_str(&campus).unwrap();

        println!("My Building: {}", building.contained_spaces[0].name);
        return Ok(Response::from("OK"));
    }
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
    app.get("/api/campi", "campi_handler", campi_handler);
    app.get("/api/<campus:string>/building",
            "get_building",
            get_building);
    // app.post("/api/create_user", "create_user", create_user);

    // Run server
    debug!("Running on 127.0.0.1:8080");
    app.run("127.0.0.1:8080");
}
