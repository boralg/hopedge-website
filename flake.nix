{
  description = "hopedge.com website";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    tomers = {
      url = "github:boralg/tomers";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    { nixpkgs, tomers, ... }:
    tomers.inputs.flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        buildFilePatterns = [
          ".*/templates/.*"
          ".*/assets/.*"
        ];
        targetPlatforms =
          let
            toolchainPackages =
              fenixPkgs: crossFenixPkgs: with fenixPkgs; [
                latest.rustfmt
                stable.rust-src
              ];
          in
          [
            {
              system = "x86_64-unknown-linux-gnu";
              arch = "x86_64-linux";
              inherit toolchainPackages;
              depsBuild = with pkgs; [ patchelf ];
              postInstall = crateName: ''
                  find $out -type f -exec sh -c '
                  if file "$1" | grep -q "ELF .* executable"; then
                    patchelf --set-interpreter "/lib64/ld-linux-x86-64.so.2" "$1"
                  fi
                ' sh {} \;
              '';
              inherit buildFilePatterns;
            }
          ];
        tomersLib = tomers.libFor system targetPlatforms;
      in
      rec {
        packagesForEachPlatform = tomersLib.packagesForEachPlatform;
        devShellsForEachPlatform = tomersLib.devShellsForEachPlatform;

        packages = packagesForEachPlatform ./.;
        devShells = devShellsForEachPlatform ./.;
      }
    );
}
