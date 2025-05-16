{
  description = "fallow: Jump King";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
      in
        with pkgs;
      {
        # `nix develop`
        devShell = mkShell {
          buildInputs = [
            cargo
            ffmpeg.dev
            llvmPackages.libclang.lib
            openssl
            pkg-config
            rustc
          ];

          shellHook = ''
            export LIBCLANG_PATH="${pkgs.llvmPackages.libclang.lib}/lib";
          '';
        };
      }
    );
}
