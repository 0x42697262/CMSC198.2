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
            dbeaver
            mariadb
            sqlitebrowser
            (pkgs.python312.withPackages (ps: with ps; [
              ipykernel
              jupyterlab
              lifelines
              matplotlib
              mysql-connector
              scikit-learn
              networkx
              notebook
              numpy
              pandas
              seaborn
              tqdm
            ]))
          ];
          shellHook = ''echo "Hello";
          fish;
          '';
        };

    };
}
