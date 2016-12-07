//! Handlers for the REST API
extern crate pencil;

use pencil::{Pencil, Request, Response, PencilResult};
use pencil::{UserError, PenUserError};

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
fn spaces_handler(_: &mut Request) -> PencilResult {
    // Get all spaces from Fenix
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
