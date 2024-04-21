{
  description = "A Nix-flake of my CMSC198.2 special problem development environment";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs, ... }:
    let
      system = "x86_64-linux";
    in
    {
      devShells."${system}".default =
        let
          pkgs = import nixpkgs {
            inherit system;
          };
        in
        pkgs.mkShell {
          packages = with pkgs; [
            mariadb
            dbeaver
            python312
          ];
          shellHook = ''echo "Hello"; fish '';
        };

    };
}
