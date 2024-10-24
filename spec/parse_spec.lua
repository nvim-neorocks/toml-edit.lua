describe("parse", function()
    local toml_edit = require("toml_edit")
    it("Can read from key", function()
        local toml_content = [[
          [rocks]
          "toml-edit" = "1.0.0"
        ]]
        local result = toml_edit.parse(toml_content)
        assert.equal("1.0.0", result.rocks["toml-edit"])
    end)
    it("Can read from table", function()
        local toml_content = [[
          [rocks.neorg]
          version = "1.0.0"
          opt = true
        ]]
        local result = toml_edit.parse(toml_content)
        local rock = result.rocks.neorg
        assert.equal("1.0.0", rock.version)
        assert.equal(true, rock.opt)
    end)
    it("Can read from inline table", function()
        local toml_content = [[
          [rocks]
          "toml-edit" = { version = "1.0.0", opt = true }
        ]]
        local result = toml_edit.parse(toml_content)
        local rock = result.rocks["toml-edit"]
        assert.equal("1.0.0", rock.version)
        assert.equal(true, rock.opt)
    end)
    it("Can read from array", function()
        local toml_content = [[
        my_arr = ["item1", "item2", "item3"]
        ]]
        local result = toml_edit.parse(toml_content)
        assert.equal("item1", result.my_arr[1])
        assert.equal("item2", result.my_arr[2])
        assert.equal("item3", result.my_arr[3])
        assert.is_nil(result.my_arr[4])
    end)
    it("Preserves inline tables", function()
        local toml_content = [[
          [rocks]
          "toml-edit" = { version = "1.0.0", opt = true }
        ]]
        local result = toml_edit.parse(toml_content)
        assert.equal(toml_content, tostring(result))
    end)
    it("Can set value", function()
        local toml_content = [[
          [rocks]
          "toml-edit" = "%s"
        ]]
        local result = toml_edit.parse(toml_content)
        result.rocks["toml-edit"] = "1.0.0"
        local expected = toml_content:format("1.0.0")
        assert.equal(expected, tostring(result))
    end)
    it("Preserves comments", function()
        local toml_content = [[
          [rocks]
          # Some comment
          "toml-edit" = "%s"
        ]]
        local result = toml_edit.parse(toml_content)
        result.rocks["toml-edit"] = "1.0.0"
        local expected = toml_content:format("1.0.0")
        assert.equal(expected, tostring(result))
    end)
    it("Can set value in table", function()
        local toml_content = [[
          [rocks.neorg]
          version = "%s"
        ]]
        local result = toml_edit.parse(toml_content)
        local rock = result.rocks.neorg
        rock.version = "2.0.0"
        local expected = toml_content:format("2.0.0")
        assert.equal(expected, tostring(result))
    end)
    it("Can set value in array", function()
        local toml_content = [[
        my_arr = ["%s", "item2"]
        ]]
        local result = toml_edit.parse(toml_content)
        result.my_arr[1] = "item1_changed"
        local expected = toml_content:format("item1_changed")
        assert.equal(expected, tostring(result))
    end)
    it("Can add value to array", function()
        local toml_content = [[
        my_arr = ["item1", "item2"]
        ]]
        local result = toml_edit.parse(toml_content)
        result.my_arr[3] = "item3"
        local new_toml_content = [[
        my_arr = ["item1", "item2", "%s"]
        ]]
        local expected = new_toml_content:format("item3")
        assert.equal(expected, tostring(result))
    end)
    it("Can add value to table", function()
        local toml_content = [[
[rocks.neorg]
version = "1.0.0"
]]
        local result = toml_edit.parse(toml_content)
        local rock = result.rocks.neorg
        rock.opt = false
        local expected = [[
[rocks.neorg]
version = "1.0.0"
opt = false
]]
        assert.equal(expected, tostring(result))
    end)
    it("Preserves inline tables when setting a value", function()
        local toml_content = [[
[rocks]
"toml-edit" = { version = "%s" }
]]
        local result = toml_edit.parse(toml_content)
        result.rocks["toml-edit"].version = "2.0.0"
        local expected = toml_content:format("2.0.0")
        assert.equal(expected, tostring(result))
    end)
    it("Preserves inline tables when adding a new value", function()
        local toml_content = [[
[rocks]
"toml-edit" = { version = "1.0.0" }
]]
        local result = toml_edit.parse(toml_content)
        result.rocks["toml-edit"].opt = true
        local expected = [[
[rocks]
"toml-edit" = { version = "1.0.0" , opt = true }
]]
        assert.equal(expected, tostring(result))
    end)
end)
