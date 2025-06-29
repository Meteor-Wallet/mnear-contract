build: lint lst mock_ft mock_validator mock_whitelist

lint:
	@cargo fmt --all
	@cargo clippy --fix --allow-dirty --allow-staged --features=test

lst: contracts/lst
	$(call local_build_wasm,lst,lst)

lst-feature-test: contracts/lst
	$(call local_build_wasm,lst,lst, "test")

mock_ft: contracts/mock_ft
	$(call local_build_wasm,mock_ft,mock_ft)

mock_validator: contracts/mock_validator
	$(call local_build_wasm,mock_validator,mock_validator)

mock_whitelist: contracts/mock_whitelist
	$(call local_build_wasm,mock_whitelist,mock_whitelist)

count:
	@tokei ./contracts/lst/src/ --files --exclude unit

release:
	$(call build_release_wasm,lst,lst)

clean:
	cargo clean
	rm -rf res/

unittest: build
ifdef TC
	cargo nextest run --package lst $(TC) -- --no-capture
else
	cargo nextest run --package lst --lib -- --failure-output immediate
endif

test: lst-feature-test mock_validator mock_whitelist
ifdef TF
	cargo nextest run --package lst --test $(TF) --no-capture
else ifdef TN
	cargo nextest run $(TN) --package lst --no-capture
else
	cargo nextest run --package lst --tests --failure-output immediate
endif

define local_build_wasm
	$(eval PACKAGE_NAME := $(1))
	$(eval WASM_NAME := $(2))
	$(eval FEATURES := $(3))

	@mkdir -p res
	@rustup target add wasm32-unknown-unknown
	@if [ -n "$(FEATURES)" ]; then \
		cargo near build non-reproducible-wasm --manifest-path ./contracts/${PACKAGE_NAME}/Cargo.toml --features=$(FEATURES) --locked; \
	else \
		cargo near build non-reproducible-wasm --manifest-path ./contracts/${PACKAGE_NAME}/Cargo.toml --locked; \
	fi
	@cp target/near/${WASM_NAME}/$(WASM_NAME).wasm ./res/$(WASM_NAME).wasm
endef

define build_release_wasm
	$(eval PACKAGE_NAME := $(1))
	$(eval WASM_NAME := $(2))

	@mkdir -p res
	@rustup target add wasm32-unknown-unknown
	@cargo near build reproducible-wasm --manifest-path ./contracts/${PACKAGE_NAME}/Cargo.toml
	@cp target/near/${WASM_NAME}/$(WASM_NAME).wasm ./res/$(WASM_NAME)_release.wasm
endef
