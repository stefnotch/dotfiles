{
  description = "Homepage for the homeserver";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-26.05";
    flake-parts.url = "github:hercules-ci/flake-parts";
    flake-parts.inputs.nixpkgs-lib.follows = "nixpkgs";

    fenix.url = "github:nix-community/fenix/monthly";
    fenix.inputs.nixpkgs.follows = "nixpkgs";
  };

  nixConfig = {
    extra-trusted-public-keys = [
      "nix-community.cachix.org-1:mB9FSh9qf2dCimDSUo8Zy7bkq5CX+/rkCWyvRCYg3Fs="
    ];
    extra-substituters = [
      "https://nix-community.cachix.org"
    ];
  };

  outputs =
    inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
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
            channel = "stable";
            date = null; # or "2026-02-24";
            sha256 = "sha256-OATSZm98Es5kIFuqaba+UvkQtFsVgJEBMmS+t6od5/U=";
          };
          rustDistributionWasm = fnx.targets.wasm32-unknown-unknown.toolchainOf {
            channel = "stable";
            date = null; # or "2026-02-24";
            sha256 = "sha256-OATSZm98Es5kIFuqaba+UvkQtFsVgJEBMmS+t6od5/U=";
          };

          rustToolchain = fnx.combine [
            rustDistribution.cargo
            rustDistribution.rust-src
            rustDistribution.rustc

            rustDistributionWasm.rust-std
          ];

          dioxus-cli = pkgs.callPackage ./nix/dioxus-cli.nix { };

          wasmBindgen = pkgs.buildWasmBindgenCli rec {
            src = pkgs.fetchCrate {
              pname = "wasm-bindgen-cli";
              version = "0.2.126";
              hash = "sha256-H6Is3fiZVxZCfOMWK5dWMSrtn50VGv0sfdnsT+cTtyk=";
            };

            cargoDeps = pkgs.rustPlatform.fetchCargoVendor {
              inherit src;
              inherit (src) pname version;
              hash = "sha256-VucqkXbCi4qtQzY/HrXiDnbSURsagPsdNVMn1Tw3UiY=";
            };
          };
        in
        {
          packages.default = pkgs.callPackage ./nix/package.nix {
            inherit
              rustToolchain
              wasmBindgen
              dioxus-cli
              ;
          };
        };
      flake = { };
    };
}
