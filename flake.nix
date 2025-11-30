{
  description = "llcc dev env";
  inputs = {
    nixpkgs = {
      url = "github:nixos/nixpkgs/nixpkgs-unstable";
    };
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
    };
    systems = {
      url = "github:nix-systems/default";
    };
    fenix = {
      url = "github:nix-community/fenix";
      inputs = {
        nixpkgs = {
          follows = "nixpkgs";
        };
      };
    };
  };

  outputs =
    inputs@{
      self,
      nixpkgs,
      flake-parts,
      systems,
      fenix,
    }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = import systems;
      perSystem =
        {
          self,
          pkgs,
          lib,
          system,
          ...
        }:
        let
          fx = fenix.packages.${system};
          toolchain = fx.latest.toolchain;
        in
        {
          devShells = {
            default = pkgs.mkShell {
              packages = [
                toolchain
              ];

              shellHook = ''
                echo -e "\033[1;32m\nmogok dev environment loaded"
                echo -e "System: ${system}"
                echo -e "rustc:   $(which rustc 2>/dev/null || echo 'not found')"
                echo -e "ra:      $(which rust-analyzer 2>/dev/null || echo 'not found')"
                echo -e "cargo:   $(which cargo 2>/dev/null || echo 'not found')\033[0m\n"
              '';
            };
          };
        };
    };
}
