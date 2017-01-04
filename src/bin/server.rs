//! Implementation of a server using Pencil and the FenixEDU API with Heroku
//! support. There are seven routes defined for the API (`/api/`).
//!
//! For room management there are only three relevant fields from the FenixEDU
//! API response: `name`, `containedSpaces` and `capacity`. A space is
//! considered a room when `containedSpaces` has zero elements.
//!
//! # REST API
//!
//! ## GET
//! * `spaces` => Returns the top level spaces from the FenixEDU API;
//! * `id/<id>` => Returns the list of contained spaces, name and capacity
//!                when relevant inside each `id`;
//! * `rooms` => Returns the rooms available to check-in and check-out of
//!              in the DB;
//! * `path/<my_path>` => Returns the contained spaces, name and capacity
//!                       when applicable for the specified hierarchical
//!                       path.
//!
//! ## POST
//! * `create_user` => Creates a user in the database;
//! * `create_room` => Adds a room to the database. A room exists when
//!                    the `contained_space` list is empty;
//! * `check_in` => Adds a user to a specified room.
//!
//! ## DELETE
//! * `check_out` => Removes a user from a specified room.
#![feature(proc_macro)]

extern crate fenix_rooms;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate pencil;

use fenix_rooms::api::handlers;
use pencil::Pencil;
use std::env;

fn get_server_port() -> u16 {
    let port_str = env::var("PORT").unwrap_or(String::new());
    port_str.parse().unwrap_or(8080)
}

fn main() {
    let mut app = Pencil::new("~/fenix-rooms/src");
    app.set_debug(true);
    app.set_log_level();
    env_logger::init().unwrap();

    // ///////////////////////////////////////////////////////
    // REST API
    // ///////////////////////////////////////////////////////

    // /////
    // GET
    // /////
    // ID
    app.get("/api/id/<id:int>", "id_handler", handlers::id_handler);
    // Spaces
    app.get("/api/spaces", "spaces_handler", handlers::spaces_handler);
    // Rooms
    app.get("/api/rooms", "rooms_handler", handlers::rooms_handler);
    // Path
    app.get("/api/path/<my_path:path>",
            "path_handler",
            handlers::path_handler);

    // /////
    // POST
    // /////
    // Create User
    app.post("/api/create_user",
             "create_user_handler",
             handlers::create_user_handler);
    // Create Room
    app.post("/api/create_room",
             "create_room_handler",
             handlers::create_room_handler);
    // Check In
    app.post("/api/check_in",
             "check_in_handler",
             handlers::check_in_handler);

    // /////
    // DELETE
    // /////
    // Check Out
    app.delete("/api/check_out",
               "check_out_handler",
               handlers::check_out_handler);

    // Run server
    let listen_addr = if env::var("DYNO").is_ok() {
        "0.0.0.0"
    } else {
        "127.0.0.1"
    };

    let ip = format!("{}:{}", listen_addr, get_server_port());
    debug!("Running on {}", ip);
    app.run(ip.as_str());
}
