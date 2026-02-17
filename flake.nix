{
  description = "Flake for Rust Development";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";

    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk/master";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      naersk,
      fenix,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };

        fenixLib = fenix.packages.${system};
        naerskLib = pkgs.callPackage naersk {};

        rustToolchain = fenixLib.latest.toolchain;
      in
      {
        packages.default = pkgs.callPackage ./default.nix {
            inherit pkgs naerskLib;
        };

        devShells.default = pkgs.mkShell {
          name = "rust";

          inputsFrom = [ self.packages.${system}.default ];

          nativeBuildInputs = [
            rustToolchain
          ];

          shellHook = ''
            DATABASE="test.sqlite3"

            if [[ ! -f $DATABASE ]]; then
                echo -e "\033[33mTesting database does not exist. Creating...\033[0m"
                touch $DATABASE
            fi
            export DATABASE_URL="sqlite:$DATABASE"

            echo -e "\033[0;32mDone!\033[0m"
          '';
        };
      }
    );
}
