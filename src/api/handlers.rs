//! Handlers for the REST API
//!
//! Each handler reads the request if need be, gets the information from getters
//! and returns a Response accordingly,
extern crate serde;

use utils;
use serde::{Serialize, Deserialize};

use super::DB_BASE_URL;
use super::hyper::status::StatusCode;
use super::hyper::client::Response as HyperResponse;
use super::pencil::{Request, Response as PencilResponse, PencilResult, UserError, PenUserError};
use super::{GenericSpace, Space, getters, misc};

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
            return Err(PenUserError(err));
        }
    };

    let buffer: String;
    let status_code: u16;

    // If the GET request is successful read the body and process the request
    if get_response.status == StatusCode::Ok {
        let body: String = match utils::read_response_body(&mut get_response) {
            Ok(buf) => buf,
            Err(err) => {
                return Err(PenUserError(UserError::new(err)));
            }
        };

        // Convert JSON to Object removing the unnecessary fields in the process
        let space: T = match utils::from_json_to_obj(&body) {
            Ok(space) => space,
            Err(err) => {
                return Err(PenUserError(UserError::new(err)));
            }
        };

        // Turn the simplified object back into JSON
        buffer = match utils::from_obj_to_json(&space) {
            Ok(json) => json,
            Err(err) => {
                return Err(PenUserError(UserError::new(err)));
            }
        };

        status_code = 200;
    } else if get_response.status == StatusCode::NotFound {
        // When the id is not valid warn the user
        status_code = 404;
        buffer = "{\"error\": \"The id was not found\"}".to_owned();
    } else {
        // When the `FenixEDU` servers are down warn the user
        status_code = 503;
        buffer = "{\"error\": \"Fenix had an error\"}".to_owned();
    }

    // Build response and set content to JSON response
    let mut response = PencilResponse::from(buffer);
    response.set_content_type("application/json");
    response.status_code = status_code;

    Ok(response)
}

/// Handler for all spaces at IST
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
        None => {
            // Build response and set content to JSON response
            let mut response = PencilResponse::from("{\"error\": \"The id \
                                                     wasn't provided\"}");
            response.set_content_type("application/json");
            response.status_code = 400;

            Ok(response)
        }
    }
}

/// Creates a User in the Database
///
/// Create a user in the database with the specified `username` in the body. If
/// multiple usernames are provided only the first will be considered.
///
/// # Arguments
/// * `request` - The request sent by the client
///
/// # Output
/// A Response with a JSON messsage and correct status code.
pub fn create_user_handler(request: &mut Request) -> PencilResult {
    // Get the username from the body of the request if it exists
    let username: &str = match request.form().get("username") {
        Some(username) => username,
        None => "",
    };

    let status_code: u16;
    let buffer: String;

    if username.is_empty() {
        status_code = 400;
        buffer = "{\"error\": \"One of the necessary arguments wasn't provided\"}".to_owned();
    } else {
        let url: String = format!("{}/users", DB_BASE_URL);
        let body: String = format!("{{\"username\": \"{}\"}}", username);

        let mut response: HyperResponse = match utils::post_request(&url, &body) {
            Ok(response) => response,
            Err(err) => {
                let error = UserError::new(err);
                return Err(PenUserError(error));
            }
        };

        if response.status == StatusCode::Ok || response.status == StatusCode::Created {
            status_code = 200;
            buffer = match utils::read_response_body(&mut response) {
                Ok(buffer) => buffer,
                Err(err) => {
                    return Err(PenUserError(UserError::new(err)));
                }
            };
        } else if response.status == StatusCode::UnprocessableEntity {
            status_code = 422;
            buffer = "{\"error\": \"The entity already exists\"}".to_owned();
        } else {
            status_code = 503;
            buffer = "{\"error\": \"There is an error in the database\"}".to_owned();
        }

    }

    let mut response = PencilResponse::from(buffer);
    response.set_content_type("application/json");
    response.status_code = status_code;

    Ok(response)
}

/// Creates a Room in the Database
///
/// Create a room in the database with the specified `id`, `capacity`,
/// `location` and `admin_id` in the body. Only the admin can create rooms in
/// the DB.
///
/// # Arguments
/// * `request` - The request sent by the client
///
/// # Output
/// A Response with a JSON messsage and correct status code.
pub fn create_room_handler(request: &mut Request) -> PencilResult {
    // Get the `fenix_id`, `capacity`, `location` and `admin_id` from the body
    // of the request if they exists. I need to clone the parameter because
    // Pencil returns a reference to the Struct and doesn't allow me to consume
    // the contents of the form. We are essentially wasting memory.
    let location: String = match request.form().get("location") {
        Some(location) => location.clone(),
        None => "".to_owned(),
    };

    let fenix_id: String = match request.form().get("fenix_id") {
        Some(fenix_id) => fenix_id.clone(),
        None => "".to_owned(),
    };

    let capacity: String = match request.form().get("capacity") {
        Some(capacity) => capacity.clone(),
        None => "".to_owned(),
    };

    let admin_id: String = match request.form().get("admin_id") {
        Some(admin_id) => admin_id.clone(),
        None => "".to_owned(),
    };

    // Default to Unauthorized.
    let mut status_code: u16 = 401;
    let mut buffer: String = "{ \"error\": \"Unauthorized access to DB\"}".to_owned();

    if admin_id == "0" {
        if location.is_empty() || capacity.is_empty() || fenix_id.is_empty() {
            status_code = 400;
            buffer = "{\"error\": \"One of the necessary arguments wasn't provided\"}".to_owned();
        } else {
            let url: String = format!("{}/rooms", DB_BASE_URL);
            let body: String = format!("{{\"location\": \"{}\", \"capacity\": {}, \"fenix_id\": \
                                       {}}}",
                                       location,
                                       capacity,
                                       fenix_id);

            let room_exists: bool = match misc::is_room(&fenix_id) {
                Ok(room_exists) => room_exists,
                Err(err) => {
                    return Err(PenUserError(err));
                }
            };

            if room_exists {
                let mut response: HyperResponse = match utils::post_request(&url, &body) {
                    Ok(response) => response,
                    Err(err) => {
                        let error = UserError::new(err);
                        return Err(PenUserError(error));
                    }
                };

                if response.status == StatusCode::Created || response.status == StatusCode::Ok {
                    buffer = match utils::read_response_body(&mut response) {
                        Ok(buffer) => buffer,
                        Err(err) => {
                            return Err(PenUserError(UserError::new(err)));
                        }
                    };
                    status_code = 200;
                } else if response.status == StatusCode::UnprocessableEntity {
                    status_code = 422;
                    buffer = "{\"error\": \"The entity already exists\"}".to_owned();
                } else {
                    status_code = 503;
                    buffer = "{\"error\": \"There is an error in the database\"}".to_owned();
                }
            } else {
                status_code = 404;
                buffer = "{ \"error\" : \"The provided id does not match a space in FenixEDU\"}"
                    .to_owned();
            }
        }
    }

    let mut response = PencilResponse::from(buffer);
    response.status_code = status_code;
    response.set_content_type("application/json");

    Ok(response)
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
pub fn check_in_handler(request: &mut Request) -> PencilResult {
    let user_id: String = match request.form().get("user_id") {
        Some(user_id) => user_id.clone(),
        None => "".to_owned(),
    };

    let room_id: String = match request.form().get("room_id") {
        Some(room_id) => room_id.clone(),
        None => "".to_owned(),
    };

    let status_code: u16;
    let buffer: String;

    if room_id.is_empty() || user_id.is_empty() {
        status_code = 400;
        buffer = "{\"error\": \"One of the necessary arguments wasn't provided\"}".to_owned();
    } else {
        let url: String = format!("{}/checkins", DB_BASE_URL);
        let body: String = format!("{{\"user_id\": {}, \"room_id\": {}}}", user_id, room_id);

        let mut response: HyperResponse = match utils::post_request(&url, &body) {
            Ok(response) => response,
            Err(err) => {
                let error = UserError::new(err);
                return Err(PenUserError(error));
            }
        };

        if response.status == StatusCode::Created || response.status == StatusCode::Ok {
            buffer = match utils::read_response_body(&mut response) {
                Ok(buffer) => buffer,
                Err(err) => {
                    return Err(PenUserError(UserError::new(err)));
                }
            };
            status_code = 200;
        } else if response.status == StatusCode::UnprocessableEntity {
            status_code = 422;
            buffer = "{\"error\": \"The entity already exists\"}".to_owned();
        } else if response.status == StatusCode::NotFound {
            status_code = 404;
            buffer = "{\"error\": \"The user_id or room_id provided was not found\"}".to_owned();
        } else {
            status_code = 503;
            buffer = "{\"error\": \"There is an error in the database\"}".to_owned();
        }
    }

    let mut response = PencilResponse::from(buffer);
    response.set_content_type("application/json");
    response.status_code = status_code;

    Ok(response)
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
pub fn check_out_handler(request: &mut Request) -> PencilResult {
    let user_id: String = match request.form().get("user_id") {
        Some(user_id) => user_id.clone(),
        None => "".to_owned(),
    };

    let room_id: String = match request.form().get("room_id") {
        Some(room_id) => room_id.clone(),
        None => "".to_owned(),
    };

    let status_code: u16;
    let buffer: String;

    if room_id.is_empty() || user_id.is_empty() {
        status_code = 400;
        buffer = "{\"error\": \"One of the necessary arguments wasn't provided\"}".to_owned();
    } else {
        let url: String = format!("{}/checkins", DB_BASE_URL);
        let body: String = format!("{{\"user_id\": {}, \"room_id\": {}}}", user_id, room_id);

        let mut response: HyperResponse = match utils::delete_request(&url, &body) {
            Ok(response) => response,
            Err(err) => {
                let error = UserError::new(err);
                return Err(PenUserError(error));
            }
        };

        // Deleting returns Ok or NoContent with no body
        if response.status == StatusCode::NoContent || response.status == StatusCode::Ok {
            status_code = 200;
            buffer = "".to_owned();
        } else if response.status == StatusCode::NotFound {
            // The resource asked to delete was not found
            status_code = 404;
            buffer = match utils::read_response_body(&mut response) {
                Ok(buffer) => buffer,
                Err(err) => {
                    return Err(PenUserError(UserError::new(err)));
                }
            };
        } else {
            // The database had an error
            status_code = 503;
            buffer = "{\"error\": \"There is an error in the database\"}".to_owned();
        }
    }

    let mut response = PencilResponse::from(buffer);
    response.set_content_type("application/json");
    response.status_code = status_code;

    Ok(response)
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
            let error = UserError::new(err);
            return Err(PenUserError(error));
        }
    };

    let buffer: String;
    let status_code: u16;

    if response.status == StatusCode::Ok {
        buffer = match utils::read_response_body(&mut response) {
            Ok(buffer) => buffer,
            Err(err) => {
                return Err(PenUserError(UserError::new(err)));
            }
        };
        status_code = 200;
    } else {
        status_code = 503;
        buffer = "{\"error\": \"There is an error in the database\"}".to_owned();
    }

    let mut response = PencilResponse::from(buffer);
    response.set_content_type("application/json");
    response.status_code = status_code;

    Ok(response)
}

/// Translate the id's to names
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
            return Err(PenUserError(UserError::new("No path provided")));
        }
    };

    // Get all spaces from Fenix
    let mut get_response: HyperResponse = match getters::get_spaces_from_id("") {
        Ok(response) => response,
        Err(err) => {
            return Err(PenUserError(err));
        }
    };

    let spaces: Space;
    let buffer: String;

    if get_response.status == StatusCode::Ok {
        buffer = match utils::read_response_body(&mut get_response) {
            Ok(buf) => buf,
            Err(err) => {
                return Err(PenUserError(UserError::new(err)));
            }
        };

    } else {
        buffer = "{\"error\": \"Fenix had an error\"}".to_owned();
    }

    spaces = match utils::from_json_to_obj(&buffer) {
        Ok(obj) => obj,
        Err(err) => {
            return Err(PenUserError(UserError::new(err)));
        }
    };

    let mut contained_spaces: Space = spaces;
    let mut my_space: GenericSpace = Default::default();

    // Search for the path in FenixEDU API
    for point in path.split('/') {
        // Send a GET request to Fenix and convert the response into an object
        my_space = match getters::search_contained_spaces(point, &contained_spaces) {
            Ok(body) => {
                match utils::from_json_to_obj(&body) {
                    Ok(spaces) => spaces,
                    Err(err) => {
                        return Err(PenUserError(UserError::new(err)));
                    }
                }
            }
            Err(err) => {
                return Err(PenUserError(err));
            }
        };

        contained_spaces = my_space.contained_spaces.clone(); // <= hate this
    }

    // Convert Object to JSON
    match utils::from_obj_to_json(&my_space) {
        Ok(json) => {
            // Build Response
            let mut response = PencilResponse::from(json);
            response.set_content_type("application/json");
            response.status_code = 200;
            Ok(response)
        }
        Err(err) => Err(PenUserError(UserError::new(err))),
    }
}
