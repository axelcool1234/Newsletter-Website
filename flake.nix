{
  description = "Newsletter Website dev shell";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-24.05";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {
          inherit system;
        };
        startDockerPostgreSQL = pkgs.writeShellScriptBin "start_docker_postgresql" ''
          set -x
          set -eo pipefail

          # Check if a custom user has been set, otherwise default to 'postgres'
          DB_USER=''${POSTGRES_USER:=postgres}

          # Check if a custom password has been set, otherwise default to 'password'
          DB_PASSWORD="''${POSTGRES_PASSWORD:=password}"

          # Check if a custom database name has been set, otherwise default to 'newsletter'
          DB_NAME="''${POSTGRES_DB:=newsletter}"

          # Check if a custom port has been set, otherwise default to '5432'
          DB_PORT="''${POSTGRES_PORT:=5432}"

          # Launch postgres using Docker
          docker run \
              -e POSTGRES_USER=''${DB_USER} \
              -e POSTGRES_PASSWORD=''${DB_PASSWORD} \
              -e POSTGRES_DB=''${DB_NAME} \
              -p "''${DB_PORT}":5432 \
              -d postgres \
              postgres -N 1000
              # ^ Increased the maximum number of connections for testing purposes
        '';
      in
        with pkgs; {
          # packages = {inherit} # inherit any defined derivations here
          devShells.default = mkShell {
            nativeBuildInputs = [
              rustc
              cargo
              rust-analyzer
              rustfmt
              clippy
              taplo # TOML formatter and lsp
              evcxr # Rust REPL
              bacon
              grcov
              alejandra # Nix Formatter

              cargo-watch
              cargo-nextest
              cargo-mutants
              cargo-spellcheck
              cargo-docset
              cargo-audit
              cargo-binutils
              cargo-tarpaulin
              cargo-deny
              cargo-make
              cargo-update
              cargo-edit
              cargo-outdated
              cargo-license
              cargo-cross
              cargo-zigbuild
              cargo-modules
              cargo-bloat
              cargo-unused-features

              lldb_18
              lld_18
              clang_18

              # Linker
              mold

              # Containerization
              docker
            ];
            buildInputs = with pkgs; [
              pkg-config # Needed for openssl Nix package
              openssl # Needed for reqwest crate

              # Scripts
              startDockerPostgreSQL
            ];
            shellHook = ''
              export HELIX_RUNTIME="$PWD/runtime"
              export OPENSSL_NO_VENDOR=1
              export OPENSSL_LIB_DIR="${pkgs.lib.getLib pkgs.openssl}/lib"
            '';
          };
        }
    );
}
