# Heroku Rocket Minimal Example

## Description

A minimal example to deploy a rocket server on heroku

## Steps if you want to start from scratch

### Initializing an heroku project using rust

1. Create a rust project and commit it to git. `cargo init` `git init`
2. Create an app on heroku with `heroku create --buildpack emk/rust your-app`
3. Create a `Procfile` and add this line to it `web: ./target/release/your-rust-app-name`. E.g for this project `web: ./target/release/heroku-rocket-minimal-example`
4. Push your app to heroku: `git push heroku master`
5. Check the logs with `heroku logs`. You should see an "Hello, World!". Great, the rust buildback is correctly building the project and running it.

### Adding a minimal rocket server

1. Add rocket as a dependency to your Cargo.toml file `rocket = "0.5.0-rc.2"`
2. Add routes to you project. See the `main.rs` for an example.
3. Be careful, you will need to bind to the port provided by Heroku thanks to env `PORT` and listen to bind to host `0.0.0.0` (localhost won't work)
4. Deploy to heroku, wait a looooong time for the crates to compile.
5. Go to you website adress, and you shoud get a 200 response with a welcoming message.
6. If it goes wrong, you can check the logs with `heroku logs --tail`
