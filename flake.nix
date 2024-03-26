{
  description = "A Nix flake for SafeHaven dev shell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    flake-compat.url = "https://flakehub.com/f/edolstra/flake-compat/1.tar.gz";
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
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [rust-overlay.overlays.default];
      };

      # NodeJS environment
      fixedNode = pkgs.nodejs_20;
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

      generateOpenApi = pkgs.writeShellScriptBin "generate_openapi" ''
        set -e

        pushd backend
        cargo run -- openapi ../frontend/openapi.json
        popd

        pushd frontend
        yarn install
        yarn generate-api
        popd
      '';

      prepareBuild = pkgs.writeShellScriptBin "prepare" ''
        set -e

        pushd backend
        sqlx database create
        sqlx migrate run
        cargo sqlx prepare
        popd

        ${generateOpenApi}/bin/generate_openapi
      '';

      testAndLint = pkgs.writeShellScriptBin "test_and_lint" ''
        set -e

        pushd backend
        cargo fmt -- --check
        cargo clippy -- -D warnings
        popd

        pushd frontend
        yarn lint
        yarn prettier
        popd
      '';
    in
      with pkgs; {
        devShells.default = mkShell {
          nativeBuildInputs = [rustChan];
          buildInputs = [
            # Prepare scripts
            prepareBuild
            testAndLint
            generateOpenApi
            # Backend
            sqlx-cli
            openssl
            pkg-config
            postgresql
            # Front
            fixedNode
            fixedNodePackages.yarn
          ];
          DATABASE_URL = "postgres://postgres:postgres@localhost:5432/safehaven";
          API_URL = "http://localhost:28669";
        };
      });
}
