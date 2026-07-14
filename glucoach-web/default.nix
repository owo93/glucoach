{
  perSystem =
    { pkgs, self', ... }:
    let
      packageJson = builtins.fromJSON (builtins.readFile ./package.json);
      pname = packageJson.name;
      version = packageJson.version;
      buildInputs = with pkgs; [
        nodejs
        pnpm_11
        pnpmConfigHook
      ];
      src = ../glucoach-web;
    in
    {
      packages.${pname} = pkgs.stdenv.mkDerivation {
        inherit pname version src;

        pnpmDeps = pkgs.fetchPnpmDeps {
          pname = "${pname}-deps";
          src = src;
          hash = "sha256-8ZPtxhGLL4KdThly/uXoY2i+eYTi+F9AMvk/QnmXJis=";
          fetcherVersion = 4;
        };

        nativeBuildInputs = buildInputs;

        buildPhase = ''
          runHook preBuild
          pnpm install \
            --offline \
            --frozen-lockfile \
            --frozen-store

          export NEXT_TELEMETRY_DISABLED=1
          pnpm next build
          runHook postBuild
        '';

        installPhase = ''
          runHook preInstall
          find .next/standalone -xtype l -delete
          mkdir -p $out
          cp -r .next/standalone/. $out/
          cp -r .next/static $out/.next/static
          cp -r public $out/public
          runHook postInstall
        '';
      };

      apps.${pname} = {
        type = "app";
        program = pkgs.writeShellApplication {
          name = pname;
          text = ''
            cd ${self'.packages.${pname}}
            ${pkgs.nodejs}/bin/node server.js
          '';
        };
      };
    };
}
