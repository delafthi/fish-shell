{
  description = "User-friendly command line shell";

  inputs = {
    # keep-sorted start
    flake-compat.url = "https://flakehub.com/f/edolstra/flake-compat/1.tar.gz";
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    treefmt-nix.url = "github:numtide/treefmt-nix";
    # keep-sorted end
  };

  outputs =
    inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [ inputs.treefmt-nix.flakeModule ];
      systems = [
        "aarch64-linux"
        "x86_64-linux"
        "aarch64-darwin"
        "x86_64-darwin"
      ];
      perSystem =
        { pkgs, ... }:
        {
          devShells = {
            default =
              with pkgs;
              mkShell {
                name = "fish-shell";
                buildInputs = [
                  cargo
                  rustc
                  rustfmt
                  rustPackages.clippy
                ];
                RUST_SRC_PATH = rustPlatform.rustLibSrc;
              };
          };
          treefmt = {
            projectRootFile = "flake.nix";
            programs = {
              keep-sorted.enable = true;
              nixfmt.enable = true;
            };
          };
        };
    };
}
