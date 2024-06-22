{
  description = "A Nix flake for SafeHaven dev shell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    alejandra = {
      url = "github:kamadorueda/alejandra/3.0.0";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    fenix,
    alejandra,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [fenix.overlays.default];
        };

        # NodeJS environment
        fixedNode = pkgs.nodejs_20;
        fixedNodePackages = pkgs.nodePackages.override {
          nodejs = fixedNode;
        };

        # Rust environment
        rustVer = fenix.packages.${system}.complete;
        rustChan = rustVer.withComponents [
          "cargo"
          "clippy"
          "rust-src"
          "rustc"
          "rustfmt"
        ];

        checkProject = pkgs.writeShellScriptBin "check_project" ''
          set -e

          pushd backend
            echo "::group::sqlx migrations checks"
              echo "Create the database"
              cargo sqlx database create

              echo "Run the migrations"
              cargo sqlx migrate run

              echo "Check the migrations"
              cargo sqlx prepare --check
            echo "::endgroup::"

            echo "::group::Backend lint"
              cargo fmt -- --check
              cargo clippy -- -D warnings
            echo "::endgroup::"

            echo "::group::OpenAPI sync checks"
              cargo run -- openapi ../frontend/calc-openapi.json
              if ! diff ../frontend/calc-openapi.json ../frontend/openapi.json; then
                echo "OpenAPI has changed, please run 'cargo run -- openapi ../frontend/openapi.json' in the frontend"
                exit 1
              fi
              rm ../frontend/calc-openapi.json
            echo "::endgroup::"
          popd

          pushd frontend
            echo "::group::Frontend checks"
              npm ci
              npm run generate-api
              npm run lint
            echo "::endgroup::"
          popd
        '';

        regenApi = pkgs.writeShellScriptBin "regen_api" ''
          set -e

          pushd backend
            cargo run -- openapi ../frontend/openapi.json
          popd

          pushd frontend
            npm run generate-api
          popd
        '';

        nginxConfigFile = pkgs.writeText "" ''
          daemon off;
          error_log stderr info;
          pid /tmp/nginx.pid;

          worker_processes auto;

          events {
            worker_connections 1024;
          }

          http {
            default_type application/octet-stream;

            sendfile on;
            keepalive_timeout 65;

            access_log /dev/stdout;

            server {
              listen 4000;

              location / {
                proxy_pass http://localhost:3000;
                proxy_set_header Host $host;
                proxy_set_header X-Real-IP $remote_addr;
                proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
                proxy_set_header X-Forwarded-Proto $scheme;

                proxy_http_version 1.1;
                proxy_set_header Upgrade $http_upgrade;
                proxy_set_header Connection "upgrade";
              }

              location /api {
                proxy_pass http://localhost:28669;
                proxy_set_header Host $host;
                proxy_set_header X-Real-IP $remote_addr;
                proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
                proxy_set_header X-Forwarded-Proto $scheme;
              }
            }
          }

        '';

        processConfigFile = pkgs.writeText "process.yaml" ''
          version: "0.5"
          processes:
            backend:
              command: cargo run -- serve
              working_dir: ./backend
            frontend:
              command: npm run dev
              working_dir: ./frontend
            reverse:
              command: ${pkgs.nginx}/bin/nginx -c ${nginxConfigFile}
        '';

        startDevEnv = pkgs.writeShellScriptBin "start_dev_env" ''
          set -e
          ${pkgs.process-compose}/bin/process-compose -f ${processConfigFile}
        '';

        # Version when compiling the packages
        version = builtins.readFile ./container_release;

        # Backend derivation
        backend =
          (pkgs.makeRustPlatform {
            cargo = rustVer.toolchain;
            rustc = rustVer.toolchain;
          })
          .buildRustPackage rec {
            inherit version;

            name = "safehaven-backend";
            src = ./backend;

            # When modifying cargo dependencies, replace the hash with pkgs.lib.fakeSha256
            # then run `nix build .#backend`. Use the hash in the error to replace the value.
            cargoSha256 = "sha256-0Rkyor7BXLI8+VZUVFqp1wMvHhBhy7sQSeOvZC8yIgE=";
          };

        # Frontend derivation
        frontend = pkgs.buildNpmPackage rec {
          inherit version;

          name = "safehaven-frontend";
          src = ./frontend;
          nodejs = fixedNode;

          # When modifying npm dependencies, replace the hash with pkgs.lib.fakeSha256
          # then run `nix build .#frontend`. Use the hash in the error to replace the value.
          npmDepsHash = "sha256-btoUwnmxoCO+vetf2uDxdN/nw3YF6H/ERLtIEtksW3A=";

          installPhase = ''
            runHook preInstall
            mkdir -p $out/usr/share/safehaven/static
            cp -rv dist/* $out/usr/share/safehaven/static
            runHook postInstall
          '';
        };

        # Docker image
        dockerImage = pkgs.dockerTools.streamLayeredImage {
          name = "ghcr.io/safehavenmaps/safehaven";
          tag = version;
          contents = [
            backend
            frontend
          ];
          config = {
            Cmd = ["/bin/safehaven" "serve"];
            Env = [
              "SH__SERVE_PUBLIC_PATH=/usr/share/safehaven/static"
            ];
            ExposedPorts = {"28669/tcp" = {};};
          };
        };
      in
        with pkgs; {
          packages = {inherit backend frontend dockerImage;};
          devShells.default = mkShell {
            buildInputs = [
              rustChan
              # Various scripts
              checkProject
              regenApi
              startDevEnv
              # Backend
              sqlx-cli
              # Front
              fixedNode
              # Nix formatting
              alejandra.defaultPackage.${system}
              # Process composing
              process-compose
            ];
            DATABASE_URL = "postgres://postgres:postgres@localhost:5432/safehaven";
          };
        }
    );
}
