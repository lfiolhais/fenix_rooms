//! Getters from the requests performed at Fenix
extern crate pencil;

use pencil::{UserError}

/// Get all spaces at IST
///
/// TODO: Description of the function
///
/// # Output
/// All spaces information with type, name and id.
fn get_spaces() -> Result<(Space, String), UserError> {

    // Send GET request to the url
    let get_response = match get_request("https://fenix.tecnico.ulisboa.pt/api/fenix/v1/spaces") {
        Ok(res) => res,
        Err(err) => {
            let error = UserError::new(err);
            return Err(error);
        }
    };

    let space: Space = serde_json::from_str(&get_response).unwrap();

    println!("Space: {:#?}", space);

    return Ok((space, get_response));
}

/// TODO: Documentation
fn get_building(request: &mut Request) -> PencilResult {
    let campus = match request.view_args.get("campus") {
        Some(campus) => campus,
        None => "",
    };

    if campus.is_empty() {
        let error = UserError::new("The campus field is empty");
        return Err(PenUserError(error));
    } else {
        let space = match get_campi() {
            Ok(space) => space.0,
            Err(err) => {
                return Err(PenUserError(err));
            }
        };

        let mut fenix_campus_id: &String = &format!("");
        // This needs to work for the other campus "Tecnológico e Nuclear"
        for c in &space {
            if c.get("name").unwrap().to_lowercase() == campus {
                fenix_campus_id = c.get("id").unwrap();
                break;
            }
        }

        if fenix_campus_id.is_empty() {
            return Ok(Response::from(format!("There was no campus found with name: {}", campus)));
        }

        println!("The id found for {} is: {}", campus, fenix_campus_id);

        let url = &format!("https://fenix.tecnico.ulisboa.pt/api/fenix/v1/spaces/{}",
                           fenix_campus_id);

        let campus = match get_request(url) {
            Ok(campus) => campus,
            Err(err) => {
                let error = UserError::new(err);
                return Err(PenUserError(error));
            }
        };

        let building: Campus = serde_json::from_str(&campus).unwrap();

        println!("My Building: {}", building.contained_spaces[0].name);
        return Ok(Response::from("OK"));
    }
}
