
build:
ifeq ($(findstring "/5.1",$(INST_LIBDIR)),)
			cargo build --release --features lua51
else ifeq ($(findstring "/5.2",$(INST_LIBDIR)),)
			cargo build --release --features lua52
else ifeq ($(findstring "/5.3",$(INST_LIBDIR)),)
			cargo build --release --features lua53
else ifeq ($(findstring "/5.4",$(INST_LIBDIR)),)
			cargo build --release --features lua54
endif

install:
	mkdir -p $(INST_LIBDIR)
	@if [ -f target/release/libtoml_edit.so ]; then cp target/release/libtoml_edit.so $(INST_LIBDIR)/toml_edit.so; fi
	@if [ -f target/release/libtoml_edit.dylib ]; then cp target/release/libtoml_edit.dylib $(INST_LIBDIR)/toml_edit.dylib; fi
	@if [ -f target/release/libtoml_edit.dll ]; then cp target/release/libtoml_edit.dll $(INST_LIBDIR)/toml_edit.dll; fi
