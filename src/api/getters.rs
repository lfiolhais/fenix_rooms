//! Getters from the requests performed at Fenix
use super::hyper::status::StatusCode;
use super::hyper::client::Response as HyperResponse;
use super::pencil::UserError;
use super::FENIX_BASE_URL;
use super::{ContainedSpace, SearchResult};
use utils;

/// Search for a space with a specified `name`
///
/// # Argument
/// * `name` => name of the space
/// * `contained_spaces` => spaces to search in
///
/// # Return Value:
/// String with the GET response or an `UserError`
pub fn search_contained_spaces(name: &str,
                               contained_spaces: &[ContainedSpace])
                               -> Result<SearchResult, UserError> {
    // Search for a the `name`
    let mut fenix_id: &str = "";
    for i in contained_spaces {
        if !i.name.is_empty() && utils::sanitize_string(&i.name) == name.to_lowercase() {
            fenix_id = &i.id;
            break;
        }
    }

    if fenix_id.is_empty() {
        return Ok(SearchResult::NotFound(format!("{} was not found", name)));
    }

    let url: String = format!("{}/{}", FENIX_BASE_URL, fenix_id);

    let mut get_response = match utils::get_request(&url) {
        Ok(response) => response,
        Err(err) => {
            return Err(UserError::new(err));
        }
    };

    let body: String;

    if get_response.status == StatusCode::Ok {
        body = match utils::read_response_body(&mut get_response) {
            Ok(buf) => buf,
            Err(err) => {
                return Err(UserError::new(err));
            }
        };
    } else {
        return Ok(SearchResult::Error("{\"error\": \"Fenix had an error\"}".to_owned()));
    }

    Ok(SearchResult::Ok(body))
}

/// Send a GET request to `FenixEDU` with the specified space `id`.
///
/// # Arguments
/// * `id` => space id.
///
/// # Output
/// Result of the transaction with a Space and String tuple and a `UserError`.
pub fn get_spaces_from_id(id: &str) -> Result<HyperResponse, UserError> {
    // Format URL
    let url: String = format!("{}/{}", FENIX_BASE_URL, id);

    // Send GET request to the url
    match utils::get_request(&url) {
        Ok(response) => Ok(response),
        Err(err) => Err(UserError::new(err)),
    }
}
