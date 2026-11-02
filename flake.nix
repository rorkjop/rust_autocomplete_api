{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    multiverse.url = "github:fzakaria/nixpkgs-multiverse";
  };
  outputs = {
    flake-utils,
    multiverse,
    self,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      mv = multiverse.multiverse.${system};
      rustPin = mv.revOf "rustc" "1.99.0";
      pkgs = mv.at rustPin;
    in {
      # xsv is not maintained and missing from nixpkgs, so we build it from source
      packages.xsv = pkgs.callPackage ./nix/packages/xsv.nix {inherit pkgs;};
      devShells.default = pkgs.mkShell {
        packages = with pkgs;
          [
            rustc
            cargo
            rustfmt
            clippy
            gzip
            unzip
            wget
          ]
          ++ [
            self.packages.${system}.xsv
          ];
        nativeBuildInputs = with pkgs; [
          openssl
          pkg-config
        ];
      };
    });
}
