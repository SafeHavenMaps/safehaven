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
        fixedNode = pkgs.nodejs_18;
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
            cargoSha256 = "sha256-VYZ69OcayjgO4aA5b/UDfnDwUBO73Ro5e1NwX3ACzrQ=";
          };

        # Frontend derivation
        frontend = pkgs.buildNpmPackage rec {
          inherit version;

          name = "safehaven-frontend";
          src = ./frontend;
          nodejs = fixedNode;

          # When modifying cargo dependencies, replace the hash with pkgs.lib.fakeSha256
          # then run `nix build .#frontend`. Use the hash in the error to replace the value.
          npmDepsHash = "sha256-PKWt4KtFNq4fFK14tKpwd4EKq9N5Eq+dmrp3FWLBNjQ=";

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
            buildInputs = [
              rustChan
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
              # Nix formatting
              alejandra.defaultPackage.${system}
            ];
            DATABASE_URL = "postgres://postgres:postgres@localhost:5432/safehaven";
            API_URL = "http://localhost:28669";
          };
        }
    );
}
