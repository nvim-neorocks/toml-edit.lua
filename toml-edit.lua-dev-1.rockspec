package = "toml-edit"
version = "dev-1"
source = {
   url = "https://github.com/vhyrro/toml-edit.lua"
}
description = {
   homepage = "https://github.com/vhyrro/toml-edit.lua",
   license = "MIT"
}

dependencies = {
    "lua >= 5.1",
    "luarocks-build-rust-mlua",
}

build = {
    type = "rust-mlua",
    modules = {
        "toml_edit"
    },
}
