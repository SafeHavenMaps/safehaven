<p align="center"><img src=".github/assets/banner.webp" alt="SafeHaven Banner"/></p>

An open-source project to create a map of safe spaces for people in need.

## Deploy

We provide docker images that are ready to deploy SafeHaven. You can find the releases in the
[packages view of the SafeHavenMaps organisation](https://github.com/SafeHavenMaps/safehaven/pkgs/container/safehaven).

### Pre-requisites

- A PostgreSQL server with PostGIS

### Prepare the database

Create a database:

```sql
CREATE DATABASE safehaven;
```

SafeHaven leverages PostgreSQL's full text search capabilities. You can help the indexer by setting a locale on the database:

```sql
ALTER DATABASE safehaven SET default_text_search_config = 'pg_catalog.french';
```

## Configure

SafeHaven is initialized with a default user named `admin` with a random password, which can be retrieved from backend logs.
The administration panel will provide you with the ability to create new users and manage the application.

To learn more about the configuration, you can visit [our documentation website](https://docs.safehavenmaps.org).

### Start the docker container:

```bash
docker run -d \
  -e SH__DATABASE__URL="postgresql://user:password@host/safehaven" \  # Set the database path
  -e SH__DATABASE__POOL_SIZE="5" \                                    # Set the number of connections to the database
  -e SH__SECURE_COOKIE="true" \                                       # Activate if you have a reverse proxy with HTTPS.
  -e SH__TOKEN_SECRET="SecretForValidatingAngSigningTokens" \         # Set a secret that will be used to sign sessions
  ghcr.io/safehavenmaps/safehaven:1.0.0                               # Change latest to the latest version, check the releases
```

## Contributing

We welcome contributions from everyone.

To spin up a development environment, follow these steps:

**Pre-requisites:**

- A working installation of Nix
  - Supports of Nix Flakes is required.
  - A PostgreSQL server with PostGIS in version 16 or higher. You can start one with `start_docker_postgresql`.

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
  - _Optional:_ Run `start_docker_postgresql` to start a PostgreSQL server with PostGIS
  - Run `start_dev_env` to start the stack using process-compose

## License

Licensed under the GNU General Public License v3.0, see [LICENSE](LICENSE).
