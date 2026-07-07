{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/26.05";

    flake-parts.url = "github:hercules-ci/flake-parts";
    flake-parts.inputs.nixpkgs-lib.follows = "nixpkgs";

    mlib.url = "github:MaxTheMooshroom/mlib.nix";
    mlib.inputs.flake-parts.follows = "flake-parts";

    flux-rs.url = "github:MaxTheMooshroom/flux-rs.nix";
  };

  outputs = { self, flux-rs, ... }@inputs:
    inputs.mlib.lib.mkFlake { inherit inputs; } (
      { config, lib, mlib, ... }:
      {
        systems = lib.systems.flakeExposed;

        imports = [ flux-rs.flakeModules.perSystem-moduleArgs ];

        perSystem =
          { self', pkgs, fluxPlatform, fluxPackages, ... }:
          {
            devShells.default = pkgs.mkShell {
              packages = with pkgs; [
                fluxPackages.flux-bins
                fluxPlatform.cargo
                cargo-workspaces
                cargo-expand
              ];
            };
        };
      }
    );
}
