//! Handlers for the REST API
use super::pencil::{Pencil, Request, Response, PencilResult};
use super::pencil::{UserError, PenUserError};
use super::getters;

/// Handler for all spaces at IST
///
/// The handler calls `get_campi()` to perform the GET request required. If the
/// request was successful its contents will be sent as JSON. Otherwise an error
/// will be sent, provided by the function.
///
/// # Arguments
/// * _ => The Pencil framework requires the handler signature to be
///        of the type `fn(&mut Request) -> PencilResult`. However, this handler
///        doesn't require access to the Request sent by the client. So we
///        declare as unused with `_`.
///
/// # Return Value
/// Error if the `get_campi()` fails. Otherwise read the contents and send it as
/// JSON.
pub fn spaces_handler(_: &mut Request) -> PencilResult {
    // Get all spaces from Fenix
    let space: String = match getters::get_spaces() {
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

pub fn campus_handler(request: &mut Request) -> PencilResult {
    // Get Campus
    let my_campus: &str = match request.view_args.get("campus") {
        Some(my_campus) => my_campus as &str,
        None => "",
    };

    if my_campus.is_empty() {
        let error = UserError::new("The campus field is empty");
        return Err(PenUserError(error));
    } else {
        let campus: String = match getters::get_campi(my_campus) {
            Ok(campus) => campus.1,
            Err(err) => {
                return Err(PenUserError(err));
            }
        };

        // Build response and set content to JSON
        let mut response = Response::from(campus);
        response.set_content_type("application/json");

        return Ok(response);
    }
}
