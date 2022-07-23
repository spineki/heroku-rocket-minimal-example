#[macro_use]
extern crate rocket;

use std::env;

use rocket::{routes, Build, Config, Rocket};

#[get("/")]
fn index() -> &'static str {
    "You reached the root of the server. It works, congratulation!"
}
#[get("/ping")]
fn ping() -> &'static str {
    "pong"
}

/// Create an launch the rocket app
#[launch]
pub async fn rocket() -> Rocket<Build> {
    let default_port = 80;

    // Getting the port provided by heroku or fallbacking to a default port
    let port: u64 = env::var("PORT")
        .and_then(|port| Ok(port.parse::<u64>().expect("Unable to parse env port")))
        .unwrap_or(default_port);

    // creating a custom rocket config with the desired port
    // notice that heroku will not bind to localhost, so we use the host (address) 0.0.0.0
    let config = Config::figment()
        .merge(("port", port))
        .merge(("address", "0.0.0.0"));

    println!("Starting the server on port {}", port);

    // creating the custom rocket object and mouting two basic routes at the root
    rocket::custom(config).mount("/", routes![index, ping])
}
