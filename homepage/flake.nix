{
  description = "Homepage for the homeserver";

  inputs = {
    devenv-root = {
      url = "file+file:///dev/null";
      flake = false;
    };
    nixpkgs.url = "github:cachix/devenv-nixpkgs/rolling";
    flake-parts.url = "github:hercules-ci/flake-parts";
    flake-parts.inputs.nixpkgs-lib.follows = "nixpkgs";
    devenv.url = "github:cachix/devenv";

    fenix.url = "github:nix-community/fenix/monthly";
    fenix.inputs.nixpkgs.follows = "nixpkgs";
  };

  nixConfig = {
    extra-trusted-public-keys = [
      "devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw="
      "nix-community.cachix.org-1:mB9FSh9qf2dCimDSUo8Zy7bkq5CX+/rkCWyvRCYg3Fs="
    ];
    extra-substituters = [
      "https://devenv.cachix.org"
      "https://nix-community.cachix.org"
    ];
  };

  outputs =
    inputs@{ flake-parts, devenv-root, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        inputs.devenv.flakeModule
      ];
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];

      perSystem =
        {
          config,
          self',
          inputs',
          pkgs,
          system,
          ...
        }:
        let
          fnx = inputs'.fenix.packages;
          rustDistribution = fnx.toolchainOf {
            channel = "nightly";
            date = null; # or "2026-02-24";
            sha256 = "sha256-kEslngyDh0HeelBSXJ/DWdEjMsce4jatUcB1mNtlRMA=";
          };
          rustDistributionWasm = fnx.targets.wasm32-unknown-unknown.toolchainOf {
            channel = "nightly";
            date = null; # or "2026-02-24";
            sha256 = "sha256-kEslngyDh0HeelBSXJ/DWdEjMsce4jatUcB1mNtlRMA=";
          };

          rustToolchain = fnx.combine [
            rustDistribution.cargo
            rustDistribution.rust-src
            rustDistribution.rustc

            rustDistributionWasm.rust-std
          ];

          dioxus-cli = pkgs.callPackage ./nix/dioxus-cli.nix { };
        in
        {

          packages.default = pkgs.callPackage ./nix/package.nix {
            inherit
              rustToolchain
              dioxus-cli
              ;
          };
        };
      flake = { };
    };
}
