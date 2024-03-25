describe("parse_as_tbl", function()
    local toml_edit = require("toml_edit")
    it("Can parse table", function()
        local toml_content = [[
          [rocks."toml-edit"]
          version = "1.0.0"
          opt = true
          [rocks."foo"]
          version = "2.0.0"
          opt = false
        ]]
        local result = toml_edit.parse_as_tbl(toml_content)
        assert.are.same({
            rocks = {
                ["toml-edit"] = {
                    version = "1.0.0",
                    opt = true,
                },
                foo = {
                    version = "2.0.0",
                    opt = false,
                },
            },
        }, result)
    end)
    it("Can loop over fields", function()
        local toml_content = [[
          [rocks."toml-edit"]
          version = "1.0.0"
          opt = true
          [rocks."foo"]
          version = "2.0.0"
          opt = false
        ]]
        local result = toml_edit.parse_as_tbl(toml_content)
        for k, v in pairs(result.rocks) do
            assert.is_not_nil(k)
            assert.is_not_nil(v)
        end
    end)
end)
