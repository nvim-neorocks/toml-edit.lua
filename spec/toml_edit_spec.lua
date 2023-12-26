describe("toml-edit", function()
    it("Can read from key", function()
        local toml_content = [[
          [rocks]
          "toml-edit" = "1.0.0"
        ]]
        local toml_edit = require("toml_edit")
        local result = toml_edit.parse(toml_content)
        assert.equal("1.0.0", result.rocks["toml-edit"])
    end)
    it("Can set value", function()
        local toml_content = [[
          [rocks]
          "toml-edit" = "%s"
        ]]
        local toml_edit = require("toml_edit")
        local result = toml_edit.parse(toml_content)
        result.rocks["toml-edit"] = "1.0.0"
        local expected = toml_content:format("1.0.0")
        assert.equal(expected, tostring(result))
    end)
end)
