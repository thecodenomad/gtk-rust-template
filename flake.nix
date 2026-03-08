{
  description = "gtk-rust-template is a gtk-rs template using blueprints";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    flake-compat = {
      url = "github:hercules-ci/flake-compat";
      flake = false;
    };
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = inputs@{ self, nixpkgs, flake-utils, flake-compat, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };

        rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

      in {


        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            cargo
            rustToolchain
            meson
            ninja
            pkg-config
            blueprint-compiler
            desktop-file-utils
            glib.bin
            gettext
          ];

          buildInputs = with pkgs; [
            gtk4
            libadwaita
            glib
            fontconfig
            freetype
          ];

          shellHook = ''
            export GSETTINGS_SCHEMA_DIR="$PWD/data"

            if [ ! -f "$PWD/data/com.example.gtk_rust_template.gschema.xml" ]; then
              cp data/com.example.gtk_rust_template.gschema.xml data/
            fi

            glib-compile-schemas "$PWD/data" 2>/dev/null || true

            export FONTCONFIG_FILE=${pkgs.fontconfig.out}/etc/fonts/fonts.conf
            export FONTCONFIG_CACHE_DIR=${pkgs.fontconfig.out}/var/cache/fontconfig
          '';
        };

        packages = {
          flatpak = pkgs.callPackage ./flatpak.nix {
            rustToolchain = rustToolchain;
            meson = pkgs.meson;
            ninja = pkgs.ninja;
          };
        };
      });
}
