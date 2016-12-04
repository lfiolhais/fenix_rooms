//! This module implements helpers for the JSON support in Pencil. This is a
//! slight modification to the `jsonify` function to allow the user to pass a
//! status code. Always sending 200 is not the correct approach,
extern crate pencil;
extern crate rustc_serialize;

use self::rustc_serialize::json;
use self::rustc_serialize::Encodable;

use self::pencil::wrappers::{Response};
use self::pencil::types::{PencilResult, PenUserError, UserError};


/// Creates a view result with the JSON representation of the given object
/// with an *application/json* mimetype. Example usage:
///
/// ```ignore
/// extern crate rustc_serialize;
///
/// use pencil::{Request, PencilResult, jsonify};
///
/// #[derive(RustcEncodable)]
/// struct User {
///     id: u8,
///     name: String,
/// }
///
/// fn get_user(_: &mut Request) -> PencilResult {
///     let user = User {
///         id: 1,
///         name: String::from("admin"),
///     };
///     return jsonify(&user);
/// }
/// ```
pub fn jsonify<T: Encodable>(object: &T, status_code: u16) -> PencilResult {
    match json::encode(object) {
        Ok(encoded) => {
            let mut response = Response::from(encoded);
            response.set_content_type("application/json");
            response.status_code = status_code;
            Ok(response)
        },
        Err(err) => {
            let error = UserError::new(format!("Json encoder error: {}", err));
            Err(PenUserError(error))
        },
    }
}
