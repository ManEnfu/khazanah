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
        buildInputs = with pkgs; [
          meson
          ninja
          toolchain
          fenix.packages.${system}.rust-analyzer
          pkg-config
          cmake
          fontconfig
          gtk4
          libxml2
          libadwaita
        ];
        shellHook = ''
          export RUST_SRC_PATH="${toolchain}/lib/rustlib/src/rust/library"
        '';
      };
    });
  };
}
