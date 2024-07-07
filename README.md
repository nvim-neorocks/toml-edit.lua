# toml-edit.lua

[![LuaRocks][luarocks-shield]][luarocks-url]

Edit toml files while preserving whitespace and formatting from Lua.

## Usage

The `parse` function creates a table with metatable,
which can be converted back to toml (preserving comments)
using `tostring`:

```lua
local toml_content = [[
[rocks]
# Some comment
"toml-edit" = "1.0.0"
]]
local toml_edit = require("toml_edit")
local toml_tbl = toml_edit.parse(toml_content)
toml_tbl.rocks["toml-edit"] = "2.0.0"
print(tostring(toml_tbl))
-- outputs:
-- [rocks]
-- # Some comment
-- "toml-edit" = "2.0.0"
```

The `parse_as_tbl` function parses toml as a regular lua table:

```lua
local toml_content = [[
[rocks]
"toml-edit" = "1.0.0"
]]
local toml_edit = require("toml_edit")
local lua_tbl = toml_edit.parse_as_tbl(toml_content)
print(tostring(toml_tbl))
-- outputs: table: 0x7ff975807668
```

> [!TIP]
>
> - Use `parse` when you need to modify the toml, and you are accessing or
>   setting fields by name.
> - Use `parse_as_tbl` when you need to perform operations that don't access
>   fields by name (e.g. iterating over key/value pairs).

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

> [!NOTE]
>
> You may need to `luarocks install --local luarocks-build-rust-mlua`.

[luarocks-shield]: https://img.shields.io/luarocks/v/neorg/toml-edit?logo=lua&color=purple&style=for-the-badge
[luarocks-url]: https://luarocks.org/modules/neorg/toml-edit
