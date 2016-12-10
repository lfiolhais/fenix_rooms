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
    // Routing
    // ///////////////////////////////////////////////////////

    // //////////////////////////////////////////////////////
    // Templates
    // The standard browser routes through here.
    // ///////////////////////////////////////////////////////

    // ///////////////////////////////////////////////////////
    // REST API
    // The REST API will only return JSON enconded responses.
    // ///////////////////////////////////////////////////////

    // Space
    app.get("/api/spaces", "spaces_handler", handlers::spaces_handler);
    // Campus
    app.get("/api/<campus:string>",
            "campus_handler",
            handlers::campus_handler);
    // Building
    app.get("/api/<campus:string>/building/<building:string>",
            "building_handler",
            handlers::building_handler);
    // Floor
    app.get("/api/<campus:string>/<building:string>/floor/<floor:int>",
            "floor_handler",
            handlers::floor_handler);
    app.get("/api/<campus:string>/<building:string>/<floor:int>/floor/<floor2:int>",
            "floor_handler",
            handlers::floor_handler);
    // Room
    app.get("/api/<campus:string>/room/<room:string>",
            "room_handler",
            handlers::room_handler);
    app.get("/api/<campus:string>/<building:string>/room/<room:string>",
            "room_handler",
            handlers::room_handler);
    app.get("/api/<campus:string>/<building:string>/<floor:int>/room/<room:string>",
            "room_handler",
            handlers::room_handler);
    app.get("/api/<campus:string>/<building:string>/<floor:int>/<floor2:int>/room/<room:string>",
            "room_handler",
            handlers::room_handler);
    // app.post("/api/create_user", "create_user", create_user);

    // Run server
    debug!("Running on 127.0.0.1:8080");
    app.run("127.0.0.1:8080");
}
