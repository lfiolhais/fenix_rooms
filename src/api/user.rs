use pencil::jsonify;

/// Creates a User in the Database
///
/// Create a user in the database with the specified username in the body. If
/// multiple usernames are provided only the first will be considered. If the no
/// username is provided an error message will be displayed with the status code
/// 204. Otherwise the username will be created and the JSON message will be
/// sent with the username and the ID given by the DB.
///
/// # Arguments
/// * request - The request sent by the client
///
/// # Output
/// A Response with a JSON messsage and correct status code.
fn create_user(request: &mut Request) -> PencilResult {
    // Get the username from the body of the request if it exists
    let username: &str = match request.form().get("username") {
        Some(username) => username,
        None => "",
    };

    // Need to change the jsonify function to accept other status codes.
    let mut object = BTreeMap::new();
    object.insert("username", username);

    let status_code: u16;
    if username.is_empty() {
        status_code = 204;
    } else {
        status_code = 201;
    }

    return jsonify(&object, status_code);
}
