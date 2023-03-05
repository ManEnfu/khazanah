{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = inputs @ { 
    self, 
    fenix, 
    nixpkgs,
    ...
  }: let
    inherit (nixpkgs) lib;
    genSystems = lib.genAttrs [
      "aarch64-linux"
      "x86_64-linux"
    ];
    pkgsFor = nixpkgs.legacyPackages;
  in {
    devShells = genSystems (system: let
      pkgs = pkgsFor.${system};
      fenixpkgs = fenix.packages.${system};
    in {
      default = let 
        toolchain = fenixpkgs.stable.withComponents [
          "cargo"
          "clippy"
          "rust-src"
          "rustc"
          "rustfmt"
        ];
      in pkgsFor.${system}.mkShell {
        name = "build-env";

        nativeBuildInputs = with pkgs; [
          wrapGAppsHook4
          meson
          ninja
          pkg-config
          toolchain
          fenix.packages.${system}.rust-analyzer
        ];

        buildInputs = with pkgs; [
          cmake
          fontconfig
          gtk4
          glib
          libxml2
          libadwaita
          gdk-pixbuf
          gsettings-desktop-schemas
        ];
        shellHook = ''
          export RUST_SRC_PATH="${toolchain}/lib/rustlib/src/rust/library"
          export RUST_LOG=debug
          export XDG_DATA_DIRS="$GSETTINGS_SCHEMAS_PATH:$XDG_DATA_DIRS"
        '';
      };
    });
  };
}
