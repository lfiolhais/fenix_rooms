extern crate hyper;

use std::io::Read;
use self::hyper::status::StatusCode;
use self::hyper::client::{Client, Response};
use self::hyper::header::{Headers, ContentType};
use self::hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};

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
pub fn get_request(url: &str) -> Result<Response, String> {
    // Create Hyper client to perform REST calls
    let client = Client::new();

    // Create and send GET request
    match client.get(url).send() {
        Ok(res) => Ok(res),
        Err(err) => Err(format!("The GET request failed with: {}", err)),
    }
}

/// Perform a POST request to the specified url
///
/// TODO
///
/// # Arguments
/// * url => Specified URL to perform the GET request to.
/// * body => Content to send
///
/// # Return Value
/// The error message of the problem or the contents of the body.
pub fn post_request(url: &str, body: &str) -> Result<Response, String> {
    // Create Hyper client to perform REST calls
    let client = Client::new();

    // Add a JSON header
    let mut headers = Headers::new();
    headers.set(ContentType(Mime(TopLevel::Application,
                                 SubLevel::Json,
                                 vec![(Attr::Charset, Value::Utf8)])));

    // Create and send POST request
    match client.post(url).headers(headers).body(body).send() {
        Ok(res) => Ok(res),
        Err(err) => Err(format!("The POST request failed with: {}", err)),
    }

}

/// Perform a DELETE request to the specified url
///
/// TODO
///
/// # Arguments
/// * url => Specified URL to perform the GET request to.
/// * body => Content to send
///
/// # Return Value
/// The error message of the problem or the contents of the body.
pub fn delete_request(url: &str, body: &str) -> Result<Response, String> {
    // Create Hyper client to perform REST calls
    let client = Client::new();

    // Add a JSON header
    let mut headers = Headers::new();
    headers.set(ContentType(Mime(TopLevel::Application,
                                 SubLevel::Json,
                                 vec![(Attr::Charset, Value::Utf8)])));

    // Create and send POST request
    match client.delete(url).headers(headers).body(body).send() {
        Ok(res) => Ok(res),
        Err(err) => Err(format!("The DELETE request failed with: {}", err)),
    }
}

/// Reads the body of the response request and returns it
///
/// The response body is read when the request has a 200 OK or 201 Created
/// status code.
///
/// # Arguments
/// * `response` => The response received from the request performed.
///
/// # Return Value
/// The contents of the body or a error message.
pub fn read_response_body(response: &mut Response) -> Result<String, String> {
    if response.status == StatusCode::Ok || response.status == StatusCode::Created {
        // Read content from response and write it to a buffer
        let mut buf: String = String::new();
        let read_size = match response.read_to_string(&mut buf) {
            Ok(size) => size,
            Err(err) => {
                let error = format!("Problem while reading message body: {}", err);
                return Err(error);
            }
        };

        if read_size != 0 {
            Ok(buf)
        } else {
            let error = format!("{{ \"error\": \"{} did not return any information\" }}", response.url);
            Ok(error)
        }
    } else {
        Ok(format!("{{ \"error\": \"The server in {} returned {}\" }}",
                   response.url,
                   response.status))
    }
}

/// Remove all accents and other non-pleasant characters from a Portuguese
/// string
///
/// This might be the dumbest code...
///
/// # Arguments
/// * string => String to convert to sane characters.
///
/// # Return Value
/// Sane String
pub fn sanitize_string(string: &str) -> String {
    string.to_lowercase()
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
        .replace("ç", "c")
}
