{ inputs, ... }:
{
  perSystem =
    { pkgs, system, ... }:
    let
      fenix = inputs.fenix.packages.${system};
      rustToolchain = fenix.stable.withComponents [
        "cargo"
        "rustc"
        "rust-src"
        "rustfmt"
        "clippy"
      ];

      craneLib = (inputs.crane.mkLib pkgs).overrideToolchain rustToolchain;

      src = craneLib.cleanCargoSource ../.;

      commonArgs = {
        inherit src;
        strictDeps = true;
      };

      cargoArtifacts = craneLib.buildDepsOnly (commonArgs // {
        pname = "glucoach-workspace-deps";
      });

      pname = "glucoach-api";
    in
    {
      _module.args = { inherit rustToolchain; };

      packages.${pname} = craneLib.buildPackage (commonArgs // {
        inherit pname cargoArtifacts;
        cargoExtraArgs = "--package ${pname}";
      });
    };
}
