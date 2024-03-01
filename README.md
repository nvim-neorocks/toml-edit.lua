# toml-edit.lua

[![LuaRocks][luarocks-shield]][luarocks-url]

Edit toml files while preserving whitespace and formatting from Lua.

## Usage

```lua
local toml_content = [[
    [rocks]
    # Some commment
    "toml-edit" = "1.0.0"
]]
local toml_edit = require("toml-edit")
local toml_tbl = toml_edit.parse(toml_content)
toml_tbl.rocks["toml-edit"] = "2.0.0"
local new_content = tostring(toml_tbl)
```

## Development

### To run tests:

Using Nix:

```console
nix flake check -L
```

Using luarocks:

```console
mkdir luarocks
luarocks make --tree=luarocks
luarocks test
```

[luarocks-shield]: https://img.shields.io/luarocks/v/neorg/toml-edit?logo=lua&color=purple&style=for-the-badge
[luarocks-url]: https://luarocks.org/modules/neorg/toml-edit
