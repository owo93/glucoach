{ inputs, ... }:
{
  perSystem =
    { pkgs, rustToolchain, ... }:
    let
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
      packages.${pname} = craneLib.buildPackage (commonArgs // {
        inherit pname cargoArtifacts;
        cargoExtraArgs = "--package ${pname}";
      });
    };
}
