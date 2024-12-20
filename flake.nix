{
  description = "Edit toml files while preserving whitespace and formatting from Lua";

  nixConfig = {
    extra-substituters = "https://neorocks.cachix.org";
    extra-trusted-public-keys = "neorocks.cachix.org-1:WqMESxmVTOJX7qoBC54TwrMMoVI1xAM+7yFin8NRfwk=";
  };

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rocks-nvim-flake.url = "github:nvim-neorocks/rocks.nvim";
    flake-parts.url = "github:hercules-ci/flake-parts";
    pre-commit-hooks = {
      url = "github:cachix/pre-commit-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs @ {
    self,
    nixpkgs,
    rocks-nvim-flake,
    flake-parts,
    pre-commit-hooks,
    ...
  }: let
    overlay = import ./nix/overlay.nix {inherit self rocks-nvim-flake;};
  in
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = [
        "x86_64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      perSystem = {
        config,
        self',
        inputs',
        pkgs,
        system,
        ...
      }: let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [overlay];
        };
        pre-commit-check = pre-commit-hooks.lib.${system}.run {
          src = self;
          hooks = {
            alejandra.enable = true;
            stylua.enable = true;
            rustfmt.enable = true;
          };
        };
      in {
        packages.default = pkgs.luajitPackages.toml-edit;

        devShells.default = pkgs.luajitPackages.toml-edit.overrideAttrs (oa: {
          name = "toml-edit devShell";

          buildInputs =
            (with pkgs; [
              rust-analyzer
              rustc
            ])
            ++ (with pre-commit-hooks.packages.${system}; [
              alejandra
              rustfmt
              stylua
              clippy
            ])
            ++ oa.buildInputs
            ++ oa.nativeBuildInputs;

          shellHook = ''
            ${oa.shellHook}
            ${self.checks.${system}.pre-commit-check.shellHook}
          '';
        });

        checks = {
          inherit pre-commit-check;
          inherit (pkgs.luajitPackages) toml-edit;
          inherit (pkgs) rocks-nvim-check;
        };
      };
      flake = {
        overlays.default = overlay;
      };
    };
}
