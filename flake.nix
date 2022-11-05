{
  description = "Development and deployment env for rusty-trombone";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
  };

  outputs = {
    devShell = pkgs.mkShell {
      packages = with pkgs; [
        openssl
        rustup
      ];
    };
  };
}
