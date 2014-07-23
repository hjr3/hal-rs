RUSTC := rustc
RUSTDOC := rustdoc

BUILDDIR := build

HAL_LIB_FILE := src/hal.rs
HAL_FILES := src/*.rs
HAL_LIB := $(foreach file,$(shell $(RUSTC) --print-file-name $(HAL_LIB_FILE)),$(BUILDDIR)/$(file))
HAL_TEST := $(BUILDDIR)/$(shell $(RUSTC) --test --print-file-name $(HAL_LIB_FILE))

$(BUILDDIR):
	mkdir -p $@

$(HAL_LIB): $(HAL_FILES) | $(BUILDDIR)
	$(RUSTC) --out-dir $(@D) $<

$(HAL_TEST): $(HAL_FILES) | $(BUILDDIR)
	$(RUSTC) --out-dir $(@D) --test $< 

all: $(HAL_LIB)

test: $(HAL_TEST)
	$(HAL_TEST)

doc: $(HAL_FILES) | $(BUILDDIR)
	$(RUSTDOC) $<

clean:
	rm -fr doc $(BUILDDIR)

.DEFAULT_GOAL := all
.PHONY: all clean
