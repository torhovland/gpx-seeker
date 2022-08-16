{
  description = "Will seek for a section of determined distance with speed above and closest to a target in a GPX file.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
    crane.inputs.nixpkgs.follows = "nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
    flake-utils.inputs.nixpkgs.follows = "nixpkgs";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, crane }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];

        pkgs = import nixpkgs {
          inherit system overlays;
          config.allowUnfree = true;
        };

        app = crane.lib.${system}.buildPackage {
          src = ./.;
          nativeBuildInputs = [ pkgs.libiconv ];
          # nativeBuildInputs = [ pkgs.pkg-config ];
          # PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
        };
      in
      with pkgs;
      {
        packages = {
          default = app;
        };
        devShell = mkShell {
          buildInputs = [
            (rust-bin.nightly.latest.default.override {
              extensions = [ "rust-src" ];
            })
            cargo
            cargo-edit
            cargo-flamegraph
            #cargo-watch
            vscode # In order for all Code Actions to work
          ];

          shellHook = ''
            XDG_DATA_DIRS=$GSETTINGS_SCHEMA_PATH
          '';
        };
      }
    );
}
