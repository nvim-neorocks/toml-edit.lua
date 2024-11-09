# Changelog

## [0.6.1](https://github.com/nvim-neorocks/toml-edit.lua/compare/v0.6.0...v0.6.1) (2024-11-09)


### Bug Fixes

* don't treat empty table assignments as arrays by default ([#44](https://github.com/nvim-neorocks/toml-edit.lua/issues/44)) ([0de0b44](https://github.com/nvim-neorocks/toml-edit.lua/commit/0de0b44b18f23d4b086b356869e858ad641c3ee3))

## [0.6.0](https://github.com/nvim-neorocks/toml-edit.lua/compare/v0.5.0...v0.6.0) (2024-10-31)


### Features

* Add support for setting/adding entire tables/arrays ([#40](https://github.com/nvim-neorocks/toml-edit.lua/issues/40)) ([caee16f](https://github.com/nvim-neorocks/toml-edit.lua/commit/caee16f01a3a09830dc5312bb84a0b5003c85a33))

## [0.5.0](https://github.com/nvim-neorocks/toml-edit.lua/compare/v0.4.1...v0.5.0) (2024-10-24)


### Features

* Add support for toml arrays ([#38](https://github.com/nvim-neorocks/toml-edit.lua/issues/38)) ([78a2adb](https://github.com/nvim-neorocks/toml-edit.lua/commit/78a2adb103116025fb353e2b5a628b30fda77b55))

## [0.4.1](https://github.com/vhyrro/toml-edit.lua/compare/v0.4.0...v0.4.1) (2024-06-27)


### Bug Fixes

* make `parse_spanned` take a table instead of a string ([fa78930](https://github.com/vhyrro/toml-edit.lua/commit/fa78930b748265949b73daa8206d092e7f43f323))

## [0.4.0](https://github.com/vhyrro/toml-edit.lua/compare/v0.3.6...v0.4.0) (2024-06-27)


### Features

* add `parse_spanned` function ([#32](https://github.com/vhyrro/toml-edit.lua/issues/32)) ([632a479](https://github.com/vhyrro/toml-edit.lua/commit/632a47966e810ced1e752606df3883f90ec7d19e))


### Bug Fixes

* fix Darwin compilation ([086f847](https://github.com/vhyrro/toml-edit.lua/commit/086f847f63811954038748dd9a981eae057a9062))

## [0.3.6](https://github.com/vhyrro/toml-edit.lua/compare/v0.3.5...v0.3.6) (2024-04-07)


### Bug Fixes

* **rockspec:** add dependencies + build_dependencies ([a9d269a](https://github.com/vhyrro/toml-edit.lua/commit/a9d269ad34b07a610fa046f00c2a552a66473263))

## [0.3.5](https://github.com/vhyrro/toml-edit.lua/compare/v0.3.4...v0.3.5) (2024-04-07)


### Bug Fixes

* **rockspec:** re-add rockspec template ([42095bf](https://github.com/vhyrro/toml-edit.lua/commit/42095bf5221a145d09028862791327312ec76723))

## [0.3.4](https://github.com/vhyrro/toml-edit.lua/compare/v0.3.3...v0.3.4) (2024-04-07)


### Reverts

* don't depend on luarocks-build-rust-mlua ([b8a29c5](https://github.com/vhyrro/toml-edit.lua/commit/b8a29c53d83fc6e4622caa5cdd4e9ada86fe3cbc))
* **luarocks:** add rockspec template ([1641c33](https://github.com/vhyrro/toml-edit.lua/commit/1641c338b5c3522d447880c38ec9606da58fb3f7))

## [0.3.3](https://github.com/vhyrro/toml-edit.lua/compare/v0.3.2...v0.3.3) (2024-04-04)


### Bug Fixes

* **luarocks:** add rockspec template ([f022e57](https://github.com/vhyrro/toml-edit.lua/commit/f022e57c308876231cc8db1a812171b521ea813f))

## [0.3.2](https://github.com/vhyrro/toml-edit.lua/compare/v0.3.1...v0.3.2) (2024-04-04)


### Bug Fixes

* set package name in luarocks.yml ([ba84b6c](https://github.com/vhyrro/toml-edit.lua/commit/ba84b6c7b976d0dedcc4c7f338091320cea7a28e))

## [0.3.1](https://github.com/vhyrro/toml-edit.lua/compare/v0.3.0...v0.3.1) (2024-04-03)


### Bug Fixes

* proper naming in luarocks.yml ([ee64612](https://github.com/vhyrro/toml-edit.lua/commit/ee64612607dd4b4ebd1f65d6e37fbdf07e8e70bc))

## [0.3.0](https://github.com/vhyrro/toml-edit.lua/compare/v0.2.1...v0.3.0) (2024-04-03)


### Features

* `parse_as_tbl` function to parse tables that can be looped over ([768e81f](https://github.com/vhyrro/toml-edit.lua/commit/768e81f746d1f59c4c178dd1223df22b6602dcf4))

## [0.2.1](https://github.com/vhyrro/toml-edit.lua/compare/v0.2.0...v0.2.1) (2024-03-04)


### Bug Fixes

* luarocks.yml workflow lacking metadata ([c10f5c8](https://github.com/vhyrro/toml-edit.lua/commit/c10f5c89ecb9b6b75511cf90c7c20181f01a3a00))
* make luarocks workflow automatic ([d3feb57](https://github.com/vhyrro/toml-edit.lua/commit/d3feb57fa12c0e76aa1ba49fe8541d5b9130faf2))

## [0.2.0](https://github.com/vhyrro/toml-edit.lua/compare/v0.1.5...v0.2.0) (2024-03-03)


### Features

* don't depend on luarocks-build-rust-mlua ([0df05df](https://github.com/vhyrro/toml-edit.lua/commit/0df05df3ee337df006f6f040db81ed1cf49ceee6))

## [0.1.5](https://github.com/vhyrro/toml-edit.lua/compare/v0.1.4...v0.1.5) (2023-12-29)


### Bug Fixes

* support inline tables ([f673182](https://github.com/vhyrro/toml-edit.lua/commit/f6731821282dbead2425215a82f3adec42fa71ac))

## [0.1.4](https://github.com/vhyrro/toml-edit.lua/compare/v0.1.3...v0.1.4) (2023-11-23)


### Bug Fixes

* fixes related to `nil` indexing and table initialization ([f6efdab](https://github.com/vhyrro/toml-edit.lua/commit/f6efdab4ca6fab276f172060971781dc42a94f2d))
* remove warnings from code ([04a2627](https://github.com/vhyrro/toml-edit.lua/commit/04a262731a7e8676bcb0bc6a5ff4156dffe6a571))

## [0.1.3](https://github.com/vhyrro/toml-edit.lua/compare/v0.1.2...v0.1.3) (2023-09-21)


### Bug Fixes

* add better error messages in __setindex, fix CI issues ([efe2ce1](https://github.com/vhyrro/toml-edit.lua/commit/efe2ce154dd4ed35e00a15a8e2cf0edd7406642b))

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
