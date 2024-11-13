{
  description = "Developing text editor in rust";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    naersk.url = "github:nix-community/naersk/master";
    utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      utils,
      naersk,
    }@inputs:
    utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        naersk-lib = pkgs.callPackage naersk { };
      in
      {
        packages.default = naersk-lib.buildPackage ./hecto; # source code is here.

        devShells.default = pkgs.mkShell {
          # CARGO_INSTALL_ROOT = "${toString ./.}/.cargo";
          # RUST_SRC_PATH = rustPlatform.rustLibSrc;

          buildInputs = with pkgs; [
            cargo
            rustc
            rust-analyzer
            rustfmt
            rustPackages.clippy
          ];
        };
      }
    );
}
