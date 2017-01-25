//! Handlers for the REST API
//!
//! Each handler reads the request if need be, gets the information from getters
//! and returns a Response accordingly,
extern crate serde;

use utils;

use serde::{Serialize, Deserialize};
use super::hyper::status::StatusCode;
use super::hyper::client::Response as HyperResponse;
use super::pencil::{Request, PencilResult};

use super::DB_BASE_URL;
use super::{GenericSpace, Space};
use super::{getters, misc};
use super::SearchResult;

// /////////////////////////////////////////////////////////////////////////////
// ID Handling
// /////////////////////////////////////////////////////////////////////////////

/// Process the provided `id`
///
/// To process the `id` a get request is sent to `FenixEDU` with it. Then the
/// status of the response is checked. If the result is Ok the contents of the
/// body are read and passed along to the client. Otherwise one of two things
/// can happen. First, the provided `id` may not be valid (it doesn't belong to
/// any space) or the `FenixEDU` servers are down. Error messages and status
/// codes are sent apropriately.
///
/// # Arguments
/// * `id` => the id of the space to get information
///
/// # Return Value
/// The JSON message processed or an error.
fn process_id<T>(id: &str) -> PencilResult
    where T: Serialize + Deserialize
{
    // Perform GET request with id
    let mut get_response: HyperResponse = match getters::get_spaces_from_id(id) {
        Ok(response) => response,
        Err(err) => {
            return Ok(misc::build_response(500, &format!("{{ \"error\": \"{}\" }}", err.desc)));
        }
    };

    let buffer: String;
    let status_code: u16;

    // If the GET request is successful read the body and process the request
    if get_response.status == StatusCode::Ok {
        let body: String = match utils::read_response_body(&mut get_response) {
            Ok(buf) => buf,
            Err(err) => {
                return Ok(misc::build_response(500, &format!("{{ \"error\": \"{}\" }}", err)));
            }
        };

        // Convert JSON to Object removing the unnecessary fields in the process
        let space: T = match utils::from_json_to_obj(&body) {
            Ok(space) => space,
            Err(err) => {
                return Ok(misc::build_response(500, &format!("{{ \"error\": \"{}\" }}", err)));
            }
        };

        // Turn the simplified object back into JSON
        buffer = match utils::from_obj_to_json(&space) {
            Ok(json) => json,
            Err(err) => {
                return Ok(misc::build_response(500, &format!("{{ \"error\": \"{}\" }}", err)));
            }
        };

        status_code = 200;
    } else if get_response.status == StatusCode::NotFound {
        // When the id is not valid warn the user
        status_code = 404;
        buffer = format!("{{\"error\": \"The id: {} was not found\"}}", id);
    } else {
        // When the `FenixEDU` servers are down warn the user
        status_code = 503;
        buffer = "{{\"error\": \"Fenix had an error\"}}".to_owned();
    }

    Ok(misc::build_response(status_code, &buffer))
}

/// Handler for the top level spaces at IST
///
/// The handler calls `utils::get_spaces_from_id()` to perform the GET request
/// required. If the request was successful its contents will be sent as JSON.
/// Otherwise an error will be sent, provided by the function.
///
/// # Arguments
/// * _ => The Pencil framework requires the handler signature to be
///        of the type `fn(&mut Request) -> PencilResult`. However, this handler
///        doesn't require access to the Request sent by the client. So we
///        declare as unused with `_`.
///
/// # Return Value
/// Error if the `utils::get_spaces_from_id()` fails. Otherwise
/// read the contents and send it as JSON.
pub fn spaces_handler(_: &mut Request) -> PencilResult {
    process_id::<Space>("")
}

/// Handler for IDs using the `FenixEDU` API. The id sent in the url will be processed.
///
/// # Arguments
/// * `id` => id to process
///
/// # Return Value
/// Error if the `get_spaces_from_id()` fails. Otherwise read the contents and
/// send it as JSON.
pub fn id_handler(request: &mut Request) -> PencilResult {
    // Get ID from request
    match request.view_args.get("id") {
        Some(id) => process_id::<GenericSpace>(id),
        None => Ok(misc::build_response(400, "{\"error\": \"The id wasn't provided\"}")),
    }
}

/// Translate the ID's to names
///
/// This handler translates id's to names letting you browse the spaces API
/// hierarchically. Previously `/api/id/<id>` and now
/// `/api/path/level1/level2/level3`. Keep in mind that by increasing the amount
/// of levels in the path the more GET requests are made. The response time is
/// now dependent on the responsiveness of the `FenixEDU` API. Also this might
/// cause a mini DDOS.
///
/// # Output
/// JSON message with the contents of the requested space.
pub fn path_handler(request: &mut Request) -> PencilResult {
    let path: String = match request.view_args.get("my_path") {
        Some(path) => path.to_owned(),
        None => {
            return Ok(misc::build_response(400, "{\"error\": \"No path provided\"}"));
        }
    };

    // Get all spaces from Fenix
    let mut get_response: HyperResponse = match getters::get_spaces_from_id("") {
        Ok(response) => response,
        Err(err) => {
            return Ok(misc::build_response(500, &format!("{{ \"error\": \"{}\" }}", err.desc)));
        }
    };

    let spaces: Space;
    let buffer: String = if get_response.status == StatusCode::Ok {
        match utils::read_response_body(&mut get_response) {
            Ok(buf) => buf,
            Err(err) => {
                return Ok(misc::build_response(500, &format!("{{ \"error\": \"{}\" }}", err)));
            }
        }
    } else {
        "{\"error\": \"Fenix had an error\"}".to_owned()
    };

    spaces = match utils::from_json_to_obj(&buffer) {
        Ok(obj) => obj,
        Err(err) => {
            return Ok(misc::build_response(500, &format!("{{ \"error\": \"{}\" }}", err)));
        }
    };

    let mut contained_spaces: Space = spaces;
    let mut my_space: GenericSpace = Default::default();

    // Search for the path in FenixEDU API
    for point in path.split('/') {
        // Send a GET request to Fenix and convert the response into an object
        my_space = match getters::search_contained_spaces(point, &contained_spaces) {
            Ok(result) => {
                let body: String = match result {
                    SearchResult::Ok(body) => body,
                    SearchResult::NotFound(msg) => {
                        return Ok(misc::build_response(404,
                                                       &format!("{{\"error\": \"{}\"}}", msg)));
                    }
                    SearchResult::Error(msg) => {
                        return Ok(misc::build_response(503,
                                                       &format!("{{\"error\": \"{}\"}}", msg)));
                    }
                };

                match utils::from_json_to_obj(&body) {
                    Ok(spaces) => spaces,
                    Err(err) => {
                        return Ok(misc::build_response(500,
                                                       &format!("{{ \"error\": \"{}\" }}", err)));
                    }
                }
            }
            Err(err) => {
                return Ok(misc::build_response(500, &format!("{{ \"error\": \"{}\" }}", err.desc)));
            }
        };

        contained_spaces = my_space.contained_spaces.clone(); // <= hate this
    }

    // Convert Object to JSON
    match utils::from_obj_to_json(&my_space) {
        Ok(json) => Ok(misc::build_response(200, &json)),
        Err(err) => Ok(misc::build_response(500, &format!("{{ \"error\": \"{}\" }}", err))),
    }
}

// /////////////////////////////////////////////////////////////////////////////
// Database Requests Handling
// /////////////////////////////////////////////////////////////////////////////

/// Create an entity in `url` with `body`
///
/// To create an entity a POST request is sent to the database. Then the status
/// of the response is checked. If the result is Ok the contents of the body are
/// read and passed along to the client. Otherwise one of three things can
/// happen. First, the object may already exist, an entity might be created that
/// doesn't exist in the `FenixEDU` API or the database is down.
/// Error messages and status codes are sent apropriately.
///
/// # Arguments
/// * `url` => the url where the entity will be created
/// * `body` => the body of the request
///
/// # Return Value
/// The JSON message processed or an error.
fn create_entity(url: &str, body: &str) -> PencilResult {
    let mut response: HyperResponse = match utils::post_request(url, body) {
        Ok(response) => response,
        Err(err) => {
            return Ok(misc::build_response(500, &format!("{{ \"error\": \"{}\" }}", err)));
        }
    };

    let buffer: String;
    let status_code: u16;

    if response.status == StatusCode::Ok || response.status == StatusCode::Created {
        status_code = 200;
        buffer = match utils::read_response_body(&mut response) {
            Ok(buffer) => buffer,
            Err(err) => {
                return Ok(misc::build_response(500, &format!("{{ \"error\": \"{}\" }}", err)));
            }
        };
    } else if response.status == StatusCode::UnprocessableEntity {
        status_code = 422;
        buffer = match utils::read_response_body(&mut response) {
            Ok(buffer) => buffer,
            Err(err) => {
                return Ok(misc::build_response(500, &format!("{{ \"error\": \"{}\" }}", err)));
            }
        };
    } else if response.status == StatusCode::NotFound {
        status_code = 404;
        buffer = match utils::read_response_body(&mut response) {
            Ok(buffer) => buffer,
            Err(err) => {
                return Ok(misc::build_response(500, &format!("{{ \"error\": \"{}\" }}", err)));
            }
        };
    } else if response.status == StatusCode::BadRequest {
        status_code = 400;
        buffer = match utils::read_response_body(&mut response) {
            Ok(buffer) => buffer,
            Err(err) => {
                return Ok(misc::build_response(500, &format!("{{ \"error\": \"{}\" }}", err)));
            }
        };
    } else {
        status_code = 503;
        buffer = "{\"error\": \"There is an error in the database\"}".to_owned();
    }

    Ok(misc::build_response(status_code, &buffer))
}

/// Creates a User in the Database
///
/// Create a user in the database with the specified `username` in the body.
///
/// # Arguments
/// * `request` - The request sent by the client
///
/// # Output
/// A Response with a JSON messsage and correct status code.
pub fn create_user_handler(mut request: &mut Request) -> PencilResult {
    if misc::is_content_type_json(request.headers().clone()) {
        // Get the username from the JSON of the request if it exists
        match misc::get_json(&mut request) {
            Some(json) => {
                match json.as_object() {
                    Some(obj) => {
                        match obj.get("username") {
                            Some(username) => {
                                if username.is_string() {
                                    let url: String = format!("{}/users", DB_BASE_URL);
                                    let body: String = format!("{{\"username\": {}}}", username);
                                    create_entity(&url, &body)
                                } else {
                                    Ok(misc::build_response(400,
                                                            "{\"error\": \"username doesn't have \
                                                             correct type\"}"))
                                }
                            }
                            None => {
                                Ok(misc::build_response(400,
                                                        "{\"error\": \"username wasn't \
                                                         provided\"}"))
                            }
                        }
                    }
                    None => Ok(misc::build_response(400, "{\"error\": \"JSON isn't an object\"}")),
                }
            }
            None => Ok(misc::build_response(500, "{\"error\": \"Failed to parse JSON\"}")),
        }
    } else {
        Ok(misc::build_response(415, "{\"error\": \"Wrong content-type used\"}"))
    }
}

/// Creates a Room in the Database
///
/// Create a room in the database with the specified `fenix_id`, `capacity`,
/// `location` and `user_id` in the body. Only the admin can create rooms in
/// the DB.
///
/// # Arguments
/// * `request` - The request sent by the client
///
/// # Output
/// A Response with a JSON messsage and correct status code.
pub fn create_room_handler(mut request: &mut Request) -> PencilResult {
    if misc::is_content_type_json(request.headers().clone()) {
        // Get the username from the JSON of the request if it exists
        match misc::get_json(&mut request) {
            Some(json) => {
                match json.as_object() {
                    Some(obj) => {
                        if obj.contains_key("location") && obj.contains_key("fenix_id") &&
                           obj.contains_key("capacity") &&
                           obj.contains_key("user_id") {
                            // Declare and initialize variables
                            let user_id: &str = match obj.get("user_id").unwrap().as_str() {
                                Some(user_id) => user_id,
                                None => "",
                            };
                            let fenix_id: &str = match obj.get("fenix_id").unwrap().as_str() {
                                Some(fenix_id) => fenix_id,
                                None => "",
                            };
                            let location: &str = match obj.get("location").unwrap().as_str() {
                                Some(location) => location,
                                None => "",
                            };
                            let capacity: &str = match obj.get("capacity").unwrap().as_str() {
                                Some(capacity) => capacity,
                                None => "",
                            };

                            if user_id.is_empty() || fenix_id.is_empty() || location.is_empty() ||
                               capacity.is_empty() {
                                return Ok(misc::build_response(400,
                                                               "{\"error\": \"parameters don't \
                                                                have correct type\"}"));
                            }

                            // Default to Unauthorized.
                            let mut status_code: u16 = 401;
                            let mut buffer: String =
                                "{ \"error\": \"Unauthorized access to database\"}".to_owned();

                            if user_id == "0" {
                                let url: String = format!("{}/rooms", DB_BASE_URL);
                                let body: String = format!("{{\"location\": \"{}\", \
                                                            \"capacity\": \"{}\", \"fenix_id\": \
                                                            \"{}\"}}",
                                                           location,
                                                           capacity,
                                                           fenix_id);

                                let room_exists: bool = match misc::is_room(fenix_id) {
                                    Ok(room_exists) => room_exists,
                                    Err(err) => {
                                        return Ok(misc::build_response(500,
                                                                       &format!("{{ \"error\": \
                                                                                 \"{}\" }}",
                                                                                err.desc)));
                                    }
                                };

                                if room_exists {
                                    return create_entity(&url, &body);
                                } else {
                                    status_code = 404;
                                    buffer = "{ \"error\" : \"The provided fenix_id does not \
                                              match a space or room in FenixEDU\"}"
                                        .to_owned();
                                }
                            }

                            Ok(misc::build_response(status_code, &buffer))
                        } else {
                            Ok(misc::build_response(400,
                                                    "{\"error\": \"One of the necessary \
                                                     arguments wasn't provided\"}"))
                        }
                    }
                    None => Ok(misc::build_response(400, "{\"error\": \"JSON isn't an object\"}")),
                }
            }
            None => Ok(misc::build_response(500, "{\"error\": \"Failed to parse JSON\"}")),
        }
    } else {
        Ok(misc::build_response(415, "{\"error\": \"Wrong content-type used\"}"))
    }
}

/// Checks in in the Database
///
/// The check in is performed with a `room_id` and a `user_id`. Then, a POST
/// request is sent and its content read and sent to the client.
///
/// # Arguments
/// * `request` - The request sent by the client
///
/// # Output
/// A Response with a JSON messsage and correct status code.
pub fn check_in_handler(mut request: &mut Request) -> PencilResult {
    if misc::is_content_type_json(request.headers().clone()) {
        // Get the username from the JSON of the request if it exists
        match misc::get_json(&mut request) {
            Some(json) => {
                match json.as_object() {
                    Some(obj) => {
                        if obj.contains_key("user_id") && obj.contains_key("room_id") {
                            let user_id: &str = match obj.get("user_id").unwrap().as_str() {
                                Some(user_id) => user_id,
                                None => "",
                            };
                            let room_id: &str = match obj.get("room_id").unwrap().as_str() {
                                Some(room_id) => room_id,
                                None => "",
                            };

                            if user_id.is_empty() || room_id.is_empty() {
                                return Ok(misc::build_response(400,
                                                               "{\"error\": \"parameters don't \
                                                                have correct type\"}"));
                            }

                            let url: String = format!("{}/checkins", DB_BASE_URL);
                            let body: String = format!("{{\"user_id\": \"{}\", \"room_id\": \
                                                        \"{}\"}}",
                                                       user_id,
                                                       room_id);
                            create_entity(&url, &body)
                        } else {
                            Ok(misc::build_response(400,
                                                    "{\"error\": \"One of the necessary \
                                                     arguments wasn't provided\"}"))
                        }
                    }
                    None => Ok(misc::build_response(400, "{\"error\": \"JSON isn't an object\"}")),
                }
            }
            None => Ok(misc::build_response(500, "{\"error\": \"Failed to parse JSON\"}")),
        }
    } else {
        Ok(misc::build_response(415, "{\"error\": \"Wrong content-type used\"}"))
    }
}

/// Checks out in the Database
///
/// The check out is performed with a `room_id` and a `user_id`. Then, a DELETE
/// request is sent and its content read and sent to the client.
///
/// # Arguments
/// * `request` - The request sent by the client
///
/// # Output
/// A Response with a JSON messsage and correct status code.
pub fn check_out_handler(mut request: &mut Request) -> PencilResult {
    if misc::is_content_type_json(request.headers().clone()) {
        // Get the username from the JSON of the request if it exists
        match misc::get_json(&mut request) {
            Some(json) => {
                match json.as_object() {
                    Some(obj) => {
                        if obj.contains_key("user_id") && obj.contains_key("room_id") {
                            let user_id: &str = match obj.get("user_id").unwrap().as_str() {
                                Some(user_id) => user_id,
                                None => "",
                            };
                            let room_id: &str = match obj.get("room_id").unwrap().as_str() {
                                Some(room_id) => room_id,
                                None => "",
                            };

                            let status_code: u16;
                            let buffer: String;

                            if room_id.is_empty() || user_id.is_empty() {
                                status_code = 400;
                                buffer = "{\"error\": \"parameters don't \
                                          have correct type\"}"
                                    .to_owned();
                            } else {
                                let url: String = format!("{}/checkins", DB_BASE_URL);
                                let body: String = format!("{{\"user_id\": \"{}\", \"room_id\": \
                                                            \"{}\"}}",
                                                           user_id,
                                                           room_id);

                                let mut response: HyperResponse =
                                    match utils::delete_request(&url, &body) {
                                        Ok(response) => response,
                                        Err(err) => {
                                            return Ok(misc::build_response(500,
                                                                           &format!("{{ \"error\": \
                                                                                     \"{}\" }}",
                                                                                    err)));
                                        }
                                    };

                                // Deleting returns Ok or NoContent with no body
                                if response.status == StatusCode::NoContent ||
                                   response.status == StatusCode::Ok {
                                    status_code = 200;
                                    buffer = "".to_owned();
                                } else if response.status == StatusCode::NotFound {
                                    // The resource asked to delete was not found
                                    status_code = 404;
                                    buffer = match utils::read_response_body(&mut response) {
                                        Ok(buffer) => buffer,
                                        Err(err) => {
                                            return Ok(misc::build_response(500,
                                                                           &format!("{{ \"error\": \
                                                                                     \"{}\" }}",
                                                                                    err)));
                                        }
                                    };
                                } else {
                                    // The database had an error
                                    status_code = 503;
                                    buffer = "{\"error\": \"There is an error in the database\"}"
                                        .to_owned();
                                }
                            }

                            Ok(misc::build_response(status_code, &buffer))
                        } else {
                            Ok(misc::build_response(400,
                                                    "{\"error\": \"One of the necessary \
                                                     arguments wasn't provided\"}"))
                        }
                    }
                    None => Ok(misc::build_response(400, "{\"error\": \"JSON isn't an object\"}")),
                }
            }
            None => Ok(misc::build_response(500, "{\"error\": \"Failed to parse JSON\"}")),
        }
    } else {
        Ok(misc::build_response(415, "{\"error\": \"Wrong content-type used\"}"))
    }

}

/// Gets the list of rooms in the Database
///
/// # Output
/// A Response with a JSON messsage and correct status code.
pub fn rooms_handler(_: &mut Request) -> PencilResult {
    let url: String = format!("{}/rooms", DB_BASE_URL);

    let mut response: HyperResponse = match utils::get_request(&url) {
        Ok(response) => response,
        Err(err) => {
            return Ok(misc::build_response(500, &format!("{{ \"error\": \"{}\" }}", err)));
        }
    };

    let buffer: String;
    let status_code: u16;

    if response.status == StatusCode::Ok {
        buffer = match utils::read_response_body(&mut response) {
            Ok(buffer) => buffer,
            Err(err) => {
                return Ok(misc::build_response(500, &format!("{{ \"error\": \"{}\" }}", err)));
            }
        };
        status_code = 200;
    } else {
        status_code = 503;
        buffer = "{\"error\": \"There is an error in the database\"}".to_owned();
    }

    Ok(misc::build_response(status_code, &buffer))
}
