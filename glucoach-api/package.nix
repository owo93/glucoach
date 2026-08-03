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

      cargoToml = fromTOML (builtins.readFile ./Cargo.toml);
      pname = cargoToml.package.name;
    in
    {
      packages.${pname} = craneLib.buildPackage commonArgs;
    };
}
