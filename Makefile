CARGO ?= cargo
CARGO_FLAGS ?=

ifeq ($(RELEASE), 1)
  CARGO_FLAGS := --release
endif

.PHONY: all
all: build

.PHONY: build
build:
	$(CARGO) build $(CARGO_FLAGS) 

.PHONY: check
check: build
	$(CARGO) check $(CARGO_FLAGS)
	$(CARGO) test $(CARGO_FLAGS) 

.PHONY: clean
clean:
	$(CARGO) clean
