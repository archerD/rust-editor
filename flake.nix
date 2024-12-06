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
        hecto-src = ./hecto;
      in
      {
        packages = rec {
            hecto = naersk-lib.buildPackage { src = hecto-src; };
            # for use with nix build (not nix run).
            test = naersk-lib.buildPackage {
                src = hecto-src;
                mode = "test";
            };

            default = hecto;
        };

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
