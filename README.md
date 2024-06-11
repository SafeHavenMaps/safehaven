<img src=".github/assets/banner.webp" alt="SafeHaven Banner"/>

An open-source project to create a map of safe spaces for people in need.

## Current status

This project is in the early stages of development. Expect bugs and missing features.

## Contributing

We welcome contributions from everyone. To spin up a development environment, follow these steps:

**Pre-requisites:**

- A working installation of Nix
  - Supports of Nix Flakes is required
- A PostgreSQL database server with PostGIS extension enabled

**Steps:**

- Clone the repository
- Run `nix develop` to enter a development environment
- Adjust the flake.nix variable to match your database database
- Inside the `backend` folder
  - Run `sqlx migrate run` to create the database schema
- Inside the `frontend` folder
  - Run `npm install` to install the dependencies
- Inside the root folder:
  - Run `regen_api` to regenerate the API
  - Run `start_dev_env` to start the stack using process-compose
