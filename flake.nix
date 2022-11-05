{
  description = "Development and deployment env for rusty-trombone";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      rec {
        devShell = pkgs.mkShell {
          packages = with pkgs; [
            openssl
            pkg-config
            rustup
          ];
        };
        shellHook = ''
          export OPENSSL_DIR=${pkgs.openssl}
        '';
      });
}
