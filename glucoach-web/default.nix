{
  perSystem =
    { pkgs, ... }:
    let
      pname = "glucoach-web";
      version = "0.1.0";
    in
    {
      packages.${pname} = pkgs.buildNpmPackage {
        inherit pname version;
        src = ../glucoach-web;
        nodejs = pkgs.nodejs;

        npmDepsHash = "sha256-yT9W/Ed8lS/h6xNxyjOWpNSzQN+AJZtgrGuN1L1XxwI=";
        npmBuildScript = "build";

        installPhase = ''
          runHook preInstall
          mkdir -p $out
          cp -r . $out/
          runHook postInstall
        '';
      };

      apps.${pname} = {
        type = "app";
        program = pkgs.writeShellApplication {
          name = pname;
          text = ''
            cd ${toString ../glucoach-web}
            ${pkgs.nodejs}/bin/npx next start
          '';
        };
      };
    };
}
