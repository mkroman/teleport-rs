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
clean: docker-clean
	$(CARGO) clean

.PHONY: docker
docker:
	@$(SHELL) ./docker/create-cluster.sh

.PHONY: docker-clean
docker-clean:
	@$(SHELL) ./docker/delete-cluster.sh
