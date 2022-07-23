# Heroku Rocket Minimal Example

## Description

A minimal example to deploy a rocket server on heroku

## Steps if you want to start from scratch

1. Create a rust project and commit it to git. `cargo init` `git init`
2. Create an app on heroku with `heroku create --buildpack emk/rust your-app`
3. Create a `Procfile` and add this line to it `web: ./target/release/your-rust-app-name`. E.g for this project `web: ./target/release/heroku-rocket-minimal-example`
4. Push your app to heroku: `git push heroku master`
5. Check the logs with `heroku logs`. You should see an "Hello, World!". Great, the rust buildback is correctly building the project and running it.
