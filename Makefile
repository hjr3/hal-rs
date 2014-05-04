RUSTC := rustc
RUSTDOC := rustdoc

BUILDDIR := build

HAL_LIB_FILE := src/hal.rs
HAL_LIB := $(foreach file,$(shell $(RUSTC) --crate-file-name $(HAL_LIB_FILE)),$(BUILDDIR)/$(file))
HAL_TEST := $(BUILDDIR)/$(shell $(RUSTC) --test --crate-file-name $(HAL_LIB_FILE))

$(BUILDDIR):
	mkdir -p $@

$(HAL_LIB): $(HAL_LIB_FILE) | $(BUILDDIR)
	$(RUSTC) --out-dir $(@D) $<

$(HAL_TEST): $(HAL_LIB_FILE) | $(BUILDDIR)
	$(RUSTC) --out-dir $(@D) --test $< 

all: $(HAL_LIB)

test: $(HAL_TEST)
	$(HAL_TEST)

doc: $(HAL_LIB_FILE) | $(BUILDDIR)
	$(RUSTDOC) $<

clean:
	rm -fr doc $(BUILDDIR)

.DEFAULT_GOAL := all
.PHONY: all clean
