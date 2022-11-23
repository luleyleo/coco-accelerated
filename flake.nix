{
  description = "Rust + CUDA Flake";

  inputs = {
    nixpkgs = {
      url = "github:NixOS/nixpkgs/6d8215281b2f87a5af9ed7425a26ac575da0438f";
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
          pkgs.futhark
          pkgs.cudaPackages.cudatoolkit_11_2
          pkgs.cudnn_cudatoolkit_11_2
          pkgs.linuxPackages.nvidia_x11
          pkgs.stdenv.cc.cc
        ];
      };
    };
}
