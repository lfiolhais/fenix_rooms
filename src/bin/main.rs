#![feature(proc_macro)]

extern crate fenix_rooms;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate serde_json;
extern crate pencil;

use fenix_rooms::api::handlers;
use pencil::Pencil;

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
    app.get("/api/<id:int>", "id_handler", handlers::id_handler);

    // Spaces
    app.get("/api/spaces", "spaces_handler", handlers::spaces_handler);

    // // Campus
    // app.get("/api/<campus:string>",
    //         "campus_handler",
    //         handlers::campus_handler);
    // // Building
    // app.get("/api/<campus:string>/building/<building:string>",
    //         "building_handler",
    //         handlers::building_handler);
    // // Floor
    // app.get("/api/<campus:string>/<building:string>/floor/<floor:string>",
    //         "floor_handler",
    //         handlers::floor_handler);
    // app.get("/api/<campus:string>/<building:string>/<floor:string>/floor/<floor2:string>",
    //         "floor_handler",
    //         handlers::floor_handler);
    // // Room
    // app.get("/api/<campus:string>/<building:string>/room/<room:string>",
    //         "room_handler",
    //         handlers::room_handler);
    // app.get("/api/<campus:string>/<building:string>/<floor:string>/room/<room:string>",
    //         "room_handler",
    //         handlers::room_handler);
    // app.get("/api/<campus:string>/<building:string>/<floor:string>/<floor2:string>/room/<room:\
    //          string>",
    //         "room_handler",
    //         handlers::room_handler);

    // /////
    // POST
    // /////
    app.post("/api/create_user",
             "create_user_handler",
             handlers::create_user_handler);
    app.post("/api/create_room",
             "create_room_handler",
             handlers::create_room_handler);
    app.post("/api/check_in",
             "check_in_handler",
             handlers::check_in_handler);
    app.post("/api/check_out",
             "check_out_handler",
             handlers::check_out_handler);

    // Run server
    debug!("Running on 127.0.0.1:8080");
    app.run("127.0.0.1:8080");
}