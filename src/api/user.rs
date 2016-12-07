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
// fn create_user(request: &mut Request) -> PencilResult {
//
//     // Get the username from the body of the request if it exists
//     let username: &str = match request.form().get("username") {
//         Some(username) => username,
//         None => "",
//     };
//
//     // Need to change the jsonify function to accept other status codes.
//     let mut object = BTreeMap::new();
//     object.insert("username", username);
//
//     // Query DB about the availability of username
//     // If username exists send a response with the 409 status code
//     // else create user
//
//     // let mut response: Response;
//     // if username.is_empty() {
//     //     response = Response::from("Username was not provided");
//     //     response.status_code = 204;
//     // } else {
//     //     response = Response::from(username);
//     //     response.status_code = 201;
//     //     response.set_content_type("application/json");
//     // }
//
//     return jsonify(&object);
// }
