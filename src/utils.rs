extern crate hyper;

use std::io::Read;
use self::hyper::client::Client;

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
