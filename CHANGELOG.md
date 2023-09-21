# Changelog

## [0.1.2](https://github.com/vhyrro/toml-edit.lua/compare/v0.1.1...v0.1.2) (2023-09-21)


### Bug Fixes

* **ci:** make luarocks workflow work ([79b15f9](https://github.com/vhyrro/toml-edit.lua/commit/79b15f95da7467e57004ef02fe6b62158dbbc690))

## [0.1.1](https://github.com/vhyrro/toml-edit.lua/compare/v0.1.0...v0.1.1) (2023-09-16)


### Bug Fixes

* `unimplemented()` -&gt; `todo()` ([a40bf6d](https://github.com/vhyrro/toml-edit.lua/commit/a40bf6de4d57a017f620fd6c163227dacc75b4c6))

## 0.1.0 (2023-09-16)


### ⚠ BREAKING CHANGES

* do not error when a key is not found, simply return `nil`

### Features

* add better error handling for __index metamethod ([1cc2d41](https://github.com/vhyrro/toml-edit.lua/commit/1cc2d4198d9ee9ccd7b8c8f2d9b3171b76a483d6))
* add support for inline tables ([48f5902](https://github.com/vhyrro/toml-edit.lua/commit/48f5902a1a206b89b5af2b018e4aee7609589320))
* initial commit ([9cbc4bf](https://github.com/vhyrro/toml-edit.lua/commit/9cbc4bf6900c1e963e64c6500111b2e792132884))
* initial working commit ([ff3f32d](https://github.com/vhyrro/toml-edit.lua/commit/ff3f32d697782ba2d71522275887c49a348af0dc))
* support `None` items, don't use `return`s ([3952fff](https://github.com/vhyrro/toml-edit.lua/commit/3952fffdb7b5a40f5640deee2cb4e2c84d244fb4))
* support arrays ([03cbecc](https://github.com/vhyrro/toml-edit.lua/commit/03cbecce7274dc0026e6a7a3cdb743f1a1f195fa))


### Bug Fixes

* change library name for mlua detection ([71ae442](https://github.com/vhyrro/toml-edit.lua/commit/71ae44241b7c51750ce70080270bad71c8d63577))
* rename rockspec to proper format ([28c1b63](https://github.com/vhyrro/toml-edit.lua/commit/28c1b638ab8507e0af9955fe31d2a7cd7511828e))


### Code Refactoring

* do not error when a key is not found, simply return `nil` ([4214ec6](https://github.com/vhyrro/toml-edit.lua/commit/4214ec610a28660c01340a008f8217fa423cc880))
