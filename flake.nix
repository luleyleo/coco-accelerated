{
  description = "Rust + CUDA Flake";

  inputs = {
    nixpkgs = {
      url = "github:NixOS/nixpkgs/22.11-beta";
    };
  };

  outputs = inputs@{ self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
        config.allowUnfree = true;
      };

    in {
      devShell.x86_64-linux = pkgs.mkShell {
        packages = [
          pkgs.cargo
          pkgs.cargo-criterion
          pkgs.futhark
          pkgs.cudaPackages.cudatoolkit
          pkgs.cudaPackages.cudnn
          pkgs.linuxPackages.nvidia_x11
          pkgs.stdenv.cc.cc
        ];
      };
    };
}
