{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    crane.url = "github:ipetkov/crane";
  };

  outputs =
    inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
        "x86_64-darwin"
      ];

      imports = [
        ./glucoach-api/default.nix
        ./glucoach-web/default.nix
      ];

      perSystem =
        {
          inputs',
          config,
          pkgs,
          ...
        }:
        let
          rustToolchain = inputs'.fenix.packages.stable.withComponents [
            "cargo"
            "clippy"
            "rust-analyzer"
            "rust-src"
            "rustfmt"
          ];
        in
        {
          _module.args = { inherit rustToolchain; };

          devShells.default = pkgs.mkShell {
            inputsFrom = [
              config.packages.glucoach-api
              config.packages.glucoach-web
            ];
            packages = [
              rustToolchain
              pkgs.cargo-watch
              pkgs.cargo-edit
              pkgs.nodejs
              pkgs.pnpm_11
              pkgs.supabase-cli
            ];
          };
        };
    };
}
