extern crate hyper;

use std::io::Read;
use self::hyper::client::Client;

/// Perform a GET request to the specified url
///
/// Build a GET request and query. Quietly bail if we fail to read the response
/// body. If the message read more than 0 bytes we can proceed to processing the
/// information. Otherwise bail quietly.
///
/// # Arguments
/// * url => Specified URL to perform the GET request to.
///
/// # Return Value
/// The error message of the problem or the contents of the body.
pub fn get_request(url: &str) -> Result<String, String> {
    // Create Hyper client to perform REST calls
    let client = Client::new();

    // Create and send GET request
    let mut res = client.get(url).send().unwrap();

    // Read content from response and write it to a buffer
    let mut buf: String = String::new();
    let read_size = match res.read_to_string(&mut buf) {
        Ok(size) => size,
        Err(err) => {
            let error = format!("Problem while reading message body: {}", err);
            return Err(error);
        }
    };

    // Bail quietly when Fenix doesn't return information
    if read_size != 0 {
        return Ok(buf);
    } else {
        let error = format!("FenixEDU did not return any information");
        return Err(error);
    }
}

/// Remove all accents from a Portuguese string
///
/// This might be the dumbest code...
///
/// # Arguments
/// * string => String to convert to sane characters.
///
/// # Return Value
/// Sane String
pub fn remove_accents(string: &String) -> String {
    let result: String = string.to_lowercase()
        .replace(" ", "-")
        .replace("/", "_")
        .replace("á", "a")
        .replace("à", "a")
        .replace("ã", "a")
        .replace("â", "a")
        .replace("é", "e")
        .replace("ê", "e")
        .replace("í", "i")
        .replace("ó", "o")
        .replace("ô", "o")
        .replace("õ", "o")
        .replace("ú", "u")
        .replace("ç", "c");

    return result;
}
