{self}: final: prev: let
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
  lua5_1 = prev.lua5_1.override {
    packageOverrides = luaPackages-override;
  };
  lua51Packages = final.lua5_1.pkgs;
in {
  inherit
    lua5_1
    lua51Packages
    ;
}
