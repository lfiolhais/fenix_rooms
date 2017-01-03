//! Getters from the requests performed at Fenix
use super::hyper::status::StatusCode;
use super::pencil::UserError;
use super::FENIX_BASE_URL;
use utils;

// /// Search for a space with a specified type and name
// ///
// /// # Argument
// /// * `type_name` => type of the space
// /// * `name` => name of the space
// /// * `contained_spaces` => spaces to search in
// ///
// /// # Return Value:
// /// String with the GET response or an `UserError`
// fn search_contained_spaces(name: &str,
//                            contained_spaces: &[ContainedSpace])
//                            -> Result<String, UserError> {
//     println!("Searching for: {}", name);
//     let mut fenix_id: &str = "";
//     for i in contained_spaces {
//         println!("TEST: {}", i.name);
//         if !i.name.is_empty() && utils::sanitize_string(&i.name) == name {
//             fenix_id = &i.id;
//             break;
//         }
//     }

//     if fenix_id.is_empty() {
//         let error = UserError::new(format!("No id found for space {}", name));
//         return Err(error);
//     }

//     let url = &format!("{}/{}", FENIX_BASE_URL, fenix_id);

//     let response = match utils::get_request(url) {
//         Ok(response) => response,
//         Err(err) => {
//             let error = UserError::new(err);
//             return Err(error);
//         }
//     };

//     Ok(response)
// }

/// Get all spaces at from Fenix
///
/// Send a GET request with the specified id.
///
/// # Output
/// Result of the transaction with a Space and String tuple and a `UserError`.
pub fn get_spaces_from_id(id: &str) -> Result<String, UserError> {
    // Format URL
    let url = &format!("{}/{}", FENIX_BASE_URL, id);

    // Send GET request to the url
    let mut get_response = match utils::get_request(url) {
        Ok(res) => res,
        Err(err) => {
            let error = UserError::new(err);
            return Err(error);
        }
    };

    match utils::read_response_body(&mut get_response, StatusCode::Ok) {
        Ok(buf) => Ok(buf),
        Err(err) => Err(UserError::new(err)),
    }

}
