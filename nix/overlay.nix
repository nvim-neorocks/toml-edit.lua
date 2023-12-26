{self}: final: prev: let
  luaPackages-override = luaself: luaprev: {
    toml-edit = luaprev.toml-edit.overrideAttrs (oa: {
      knownRockspec = "${self}/toml-edit-dev-1.rockspec";
      src = self;
      cargoDeps = final.rustPlatform.importCargoLock {
        lockFile = self + "/Cargo.lock";
      };
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
