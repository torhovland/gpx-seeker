{
  description = "Will seek for a section of determined length with speed above and closest to a target in a GPX file.";

  inputs = {
    nixpkgs.url      = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
          config.allowUnfree = true;
        };
      in
      with pkgs;
      {
        devShell = mkShell {
          buildInputs = [
            (rust-bin.nightly.latest.default.override {
              extensions = [ "rust-src" ];
            })
            cargo-edit
            cargo-flamegraph
            cargo-watch
            vscode # In order for all Code Actions to work
          ];

          shellHook = ''
            XDG_DATA_DIRS=$GSETTINGS_SCHEMA_PATH
          '';
        };
      }
    );
}
