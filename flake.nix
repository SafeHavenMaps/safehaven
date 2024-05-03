{
  description = "A Nix flake for SafeHaven dev shell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [rust-overlay.overlays.default];
        };

        # NodeJS environment
        fixedNode = pkgs.nodejs_18;
        fixedNodePackages = pkgs.nodePackages.override {
          nodejs = fixedNode;
        };

        # Rust environment
        rustVer = pkgs.rust-bin.stable."1.77.0";
        rustChan = rustVer.default.override {
          targets = [];
          extensions = [
            "clippy"
            "rust-src"
            "rustc-dev"
            "rustfmt"
            "rust-analyzer"
          ];
        };

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

        # Version when compiling the packages
        version = builtins.readFile ./container_release;

        # Backend derivation
        rustPlatform = with pkgs;
          makeRustPlatform {
            cargo = rustChan;
            rustc = rustChan;
          };
        backend = rustPlatform.buildRustPackage rec {
          inherit version;

          name = "safehaven-backend";
          src = ./backend;
          cargoSha256 = "sha256-ONMhePZCjS3SX53POAcTDVbMRF8WWicrtMxx7GFFZjk=";
        };

        # Frontend derivation
        frontend = pkgs.buildNpmPackage rec {
          inherit version;

          name = "safehaven-frontend";
          src = ./frontend;
          nodejs = fixedNode;

          npmDepsHash = "sha256-RwwKTcnb8sq7kz28rUKIcbiydtjAnE6tFJsbkrd9gsY=";

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
            Cmd = ["/bin/safehaven"];
            Env = [
              "SAFEHAVEN_serve_public_path=/usr/share/safehaven/static"
            ];
            ExposedPorts = {"28669/tcp" = {};};
          };
        };
      in
        with pkgs; {
          packages = {inherit backend frontend dockerImage;};
          devShells.default = mkShell {
            nativeBuildInputs = [rustChan];
            buildInputs = [
              # Various scripts
              checkProject
              regenApi
              # Backend
              sqlx-cli
              openssl
              pkg-config
              postgresql
              # Front
              fixedNode
            ];
            DATABASE_URL = "postgres://postgres:postgres@localhost:5432/safehaven";
            API_URL = "http://localhost:28669";
          };
        }
    );
}
