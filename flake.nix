{
  description = "Developing text editor in rust";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    naersk.url = "github:nix-community/naersk/master";
  };

  outputs = { self, nixpkgs, naersk }@inputs:
  let 
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
    naersk-lib = pkgs.callPackage naersk { };
  in {
    packages.${system}.default = naersk-lib.buildPackage ./hecto; # source code is here.

    devShells.${system}.default = pkgs.mkShell {
      # CARGO_INSTALL_ROOT = "${toString ./.}/.cargo";
      # RUST_SRC_PATH = rustPlatform.rustLibSrc;

      buildInputs = with pkgs; [ cargo rustc rust-analyzer rustfmt rustPackages.clippy ];
    };
  };
}
