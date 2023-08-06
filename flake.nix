{
  description = "Rust + CUDA Flake";

  inputs = {
    nixpkgs = {
      url = "github:NixOS/nixpkgs/23.05";
    };
    nixpkgsOld.url =
      # Tensorflow is currently a little iffy. This is a working nixpkgs rev
      # (taken from gerczuma's flake.lock).
      "github:NixOS/nixpkgs/bc5d68306b40b8522ffb69ba6cff91898c2fbbff";

  };

  outputs = inputs@{ self, nixpkgs, nixpkgsOld }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
        config.allowUnfree = true;
      };
      pkgsOld = import nixpkgsOld {
        inherit system;
        config.allowUnfree = true;
      };

    cudatoolkit = pkgsOld.cudaPackages.cudatoolkit_11_2;
    cudnn = pkgsOld.cudnn_cudatoolkit_11_2;
    nvidia = pkgsOld.linuxPackages.nvidia_x11;


    in {
      devShell.x86_64-linux = pkgs.mkShell {
        shellHook = ''
          export LD_LIBRARY_PATH="${
            pkgs.lib.makeLibraryPath [
              pkgs.cargo
              pkgs.libclang
              cudatoolkit
              cudatoolkit.lib
              cudatoolkit.out
              nvidia
              pkgs.stdenv.cc.cc
            ]
          }:$LD_LIBRARY_PATH";
          unset SOURCE_DATE_EPOCH
          export CPATH="${cudatoolkit.out}/include:$CPATH"
          export LIBRARY_PATH="${cudatoolkit.out}/lib64:$LIBRARY_PATH"
          export CUDA_HOME="${cudatoolkit.out}"
        '';

        packages = [
          pkgs.cargo
          pkgs.cargo-criterion
          pkgs.rustfmt
          pkgs.clang
          pkgs.libclang
          pkgs.futhark
          cudatoolkit
          cudatoolkit.lib
          nvidia
          pkgs.stdenv.cc.cc
        ];
      };
    };
}
