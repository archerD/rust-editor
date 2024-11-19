# Hecto text editor

Creating rust text editor, following (at least roughly) tutorial on [flenker.blog](archive.flenker.blog/hecto)

## building/nix
Setup to build and run using nix flakes: `nix run` should be sufficent on a system with nix installed and flakes enabled.

### arguments
It seems that arguments are not being passed when using `nix run -- args`...
One workaround would be to run `nix develop` and then in the resulting shell, use `cargo run args`.
Or alternatively, specifying the flake to run seems to work: `nix run path/to/flake -- args`.

