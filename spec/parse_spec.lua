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
    it("Can interatively build table", function()
        local toml_content = [[]]
        local result = toml_edit.parse(toml_content)
        result.rocks = {}
        result.rocks.neorg = {}
        result.rocks.neorg.version = "1.0.0"
        local expected = [[
[rocks]

[rocks.neorg]
version = "1.0.0"
]]
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
    it("Can set entire table", function()
        local toml_content = [[
[rocks]

[rocks.neorg]
version = "1.0.0"
]]
        local result = toml_edit.parse(toml_content)
        result.rocks = {
            neorg = { version = "2.0.0", pin = true },
        }
        local result_str = tostring(result)
        assert.is_not_nil(result_str:find("%[rocks.neorg]"))
        assert.is_not_nil(result_str:find('version = "2.0.0"'))
        assert.is_not_nil(result_str:find("pin = true"))
    end)
    it("Can add entire table", function()
        local result = toml_edit.parse([[]])
        result.rocks = {
            neorg = { version = "2.0.0", pin = true },
        }
        local result_str = tostring(result)
        assert.is_not_nil(result_str:find("%[rocks.neorg]"))
        assert.is_not_nil(result_str:find('version = "2.0.0"'))
        assert.is_not_nil(result_str:find("pin = true"))
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
        local expected = [[
my_arr = ["item1", "item2", "item3"]
]]
        assert.equal(expected, tostring(result))
    end)
    it("Can set entire array", function()
        local toml_content = [[
my_arr = ["item1", "item2"]
]]
        local result = toml_edit.parse(toml_content)
        result.my_arr = { "new_item1", "new_item2" }
        local expected = [[
my_arr = ["new_item1", "new_item2"]
]]
        assert.equal(expected, tostring(result))
    end)
    it("Cannot use complex types in lists", function()
        local result = toml_edit.parse([[]])
        assert.error(function()
            result.my_arr = { { "new_item1" }, { "new_item2" } }
        end)
        assert.equal([[]], tostring(result))
    end)
    it("Can add entire array", function()
        local result = toml_edit.parse([[]])
        result.my_arr = { "new_item1", "new_item2" }
        local expected = [[
my_arr = ["new_item1", "new_item2"]
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
