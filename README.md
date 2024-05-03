# SafeHaven

An open-source project to create a map of safe spaces for people in need.

## Current status

This project is in the early stages of development. Expect bugs and missing features.

## Contributing

We welcome contributions from everyone. To spin up a development environment, follow these steps:

__Pre-requisites:__

- A working installation of Nix
  - Supports of Nix Flakes is required
- A PostgreSQL database server with PostGIS extension enabled

__Steps:__

- Clone the repository
- Run `nix develop` to enter a development environment
- Adjust the flake.nix variable to match your database database
- Inside the `backend` folder
  - Run `sqlx migrate run` to create the database schema
  - Run `cargo run -- serve` to start the backend server
- Inside the `frontend` folder 
  - Run `npm install` to install the dependencies
  - Run `npm run generate-api` to generate the API client using the openapi definition
  - Run `npm run dev` to start the development server
