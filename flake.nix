{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
    systems.url = "github:nix-systems/default";
    flake-utils.url = "github:numtide/flake-utils";
    devenv.url = "github:cachix/devenv";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, devenv, systems, flake-utils, fenix, ... }@inputs:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        channel = fenix.packages.${pkgs.system}.latest;
      in {
        packages = {
          devenv-up = self.devShells.${system}.default.config.procfileScript;
        };

        devShells = {
          default = devenv.lib.mkShell {
            inherit inputs pkgs;
            modules = [{
              # https://devenv.sh/reference/options/
              packages = with pkgs;
              # [ cargo-watch pkg-config cmake channel.rust-analyzer ]
                [ cargo-expand cargo-watch pkg-config cmake ]
                ++ lib.optionals stdenv.isDarwin (with darwin.apple_sdk; [
                  libiconv
                  frameworks.Security
                  frameworks.CoreFoundation
                  frameworks.SystemConfiguration
                ]);

              difftastic.enable = true;
              languages.nix.enable = true;
              # languages.rust = {
              #   enable = true;
              #   toolchain = channel.toolchain;
              # };
            }];
          };
        };
      });

  nixConfig = {
    extra-trusted-public-keys =
      "devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw=";
    extra-substituters = "https://devenv.cachix.org";
  };
}
