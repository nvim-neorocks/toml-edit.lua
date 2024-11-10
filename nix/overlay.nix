{
  self,
  rocks-nvim-flake,
}: final: prev: let
  luaPackages-override = luaself: luaprev: {
    toml-edit = luaprev.toml-edit.overrideAttrs (oa: {
      knownRockspec = "${self}/toml-edit-dev-1.rockspec";
      src = self;
      cargoDeps = final.rustPlatform.importCargoLock {
        lockFile = self + "/Cargo.lock";
      };
      nativeCheckInputs = with luaself;
        [
          luarocks-build-rust-mlua
          busted
        ]
        ++ oa.nativeCheckInputs;
      propagatedBuildInputs =
        # HACK: This shouldn't be necessary, but it appears to be.
        with luaself;
          [
            busted
          ]
          ++ oa.propagatedBuildInputs;
      preCheck = ''
        mkdir luarocks
        luarocks $LUAROCKS_EXTRA_ARGS make --tree=luarocks --deps-mode=all
      '';
      doCheck = true;
    });
  };
  luajit = prev.luajit.override {
    packageOverrides = luaPackages-override;
  };
  luajitPackages = final.luajit.pkgs;
in {
  inherit
    luajit
    luajitPackages
    ;

  rocks-nvim-check = rocks-nvim-flake.checks.${final.system}.integration-nightly.overrideAttrs (oa: {
    propagatedBuildInputs =
      final.lib.filter (pkg: pkg.pname != "toml-edit") oa.propagatedBuildInputs
      ++ [final.luajitPackages.toml-edit];
  });
}
