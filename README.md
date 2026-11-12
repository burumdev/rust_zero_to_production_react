# Zero to Production in Rust + React

This is an implementation of the Actix-web newsletter app based on Luca Palmieri's book of the same name. A single page application React frontend is added as a user interface. It's a hands-on experience in professional Rust web stack and modern typescript frontend tooling.

The book version used as reference is 2021. Some of the libraries had subtle changes in their APIs, feature flags and types in current versions. For example instead of `once_cell` crate for the Lazy initialization in integration tests, I used `LazyLock` from `std::sync`, as it is stabilized now.
Some of the libraries used in the book are used with original downgraded versions for compatibility and interoperability. As a result of it, this codebase might not have the latest security advisory patches applied and shouldn't be used or derived from in production (despite the name).

## Differences with the original project
* Added a UI in `frontend` directory built with React, Rsbuild, Tanstack Query for React, React Router and Tailwindcss.
* Added docker compose setup for easier ops and communication between app docker and database docker in local development environment. So it is easy to test the app in both development and production mode in local.
* Mailtrap API for email testing instead of Postmark. The free tier supports only text based emails.
* Email testing is done based on direct response from Mailtrap server instead of a mock server.
Successful mailtrap API send requests return a list of `message_id`s (one for single emails) and this `message_id` is used to further query the mailtrap API for the text contents of the email in test inbox (sandbox). I think this is a better and more realistic way to test email based user stories.
* Since frontend is SPA, mutation requests are JSON based instead of forms
* CSRF protection is handled with a custom HMAC middleware
* The `modname/mod.rs` directory structure is not used. The more preferred approach `modname.rs` in root and submodules in `modname` directory is used.
* DigitalOcean deployment setup is omitted
* SSL support is disabled

# Running
Before running the app, make a copy of `.env-example` file as `.env` and fill in the credentials
with your local credentials. Some of the items in the file are mailtrap specific and based on their API.
There's also another `.env-example` for the frontend in `frontend` directory that will need the same.
* You'll need `psql` on your system and `sqlx-cli` on your Rust installation.
* You'll need `Node.js` and `pnpm` installed for the frontend build.
* run `pnpm install` in `frontend` directory to pull frontend dependencies.

## Running on the local machine (development mode)
* Run `./scripts/init_db.sh`. This will also migrate the database
* Run `cargo run`
* A visit to http://localhost:8000 (production frontend) is necessary to set the CSRF cookie and session cookie upon succesful login.
* In `frontend` directory run `pnpm run dev` to launch frontend development server

## Running containerized in Docker (production mode)
* Run `docker compose up`
* Run `sqlx migrate run`
* Navigate to `http://localhost:8000` to view the production build of the frontend
