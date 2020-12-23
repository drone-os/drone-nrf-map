features := 'bit-band uarte'
target := `drone print target 2>/dev/null || echo ""`

# Install dependencies
deps:
	type cargo-readme >/dev/null || cargo +stable install cargo-readme
	type drone >/dev/null || cargo install drone
	rustup target add $(drone print target)
	rustup component add clippy
	rustup component add rustfmt

# Reformat the source code
fmt:
	cargo fmt

# Check the source code for mistakes
lint:
	cargo clippy --package drone-nrf-map-svd \
		--target=$(rustc --version --verbose | sed -n '/host/{s/.*: //;p}')
	cargo clippy --all --exclude drone-nrf-map-svd --features "{{features}}"

# Build the documentation
doc:
	cargo doc --package drone-nrf-map-svd \
		--target=$(rustc --version --verbose | sed -n '/host/{s/.*: //;p}')
	cargo doc --package drone-nrf-map --features "{{features}}"

# Open the documentation in a browser
doc-open: doc
	cargo doc --package drone-nrf-map --features "{{features}}" --open

# Run the tests
test:
	cargo test --package drone-nrf-map --features "{{features}} std" \
		--target=$(rustc --version --verbose | sed -n '/host/{s/.*: //;p}')

# Test all MCUs
test-all:
	RUSTFLAGS='--cfg cortexm_core="cortexm4f_r0p1" --cfg nrf_mcu="nrf52810"' cargo test --package drone-nrf-map --features "{{features}} std" --target=$(rustc --version --verbose | sed -n '/host/{s/.*: //;p}')
	RUSTFLAGS='--cfg cortexm_core="cortexm4f_r0p1" --cfg nrf_mcu="nrf52811"' cargo test --package drone-nrf-map --features "{{features}} std" --target=$(rustc --version --verbose | sed -n '/host/{s/.*: //;p}')
	RUSTFLAGS='--cfg cortexm_core="cortexm4f_r0p1" --cfg nrf_mcu="nrf52832"' cargo test --package drone-nrf-map --features "{{features}} std" --target=$(rustc --version --verbose | sed -n '/host/{s/.*: //;p}')
	RUSTFLAGS='--cfg cortexm_core="cortexm4f_r0p1" --cfg nrf_mcu="nrf52840"' cargo test --package drone-nrf-map --features "{{features}} std" --target=$(rustc --version --verbose | sed -n '/host/{s/.*: //;p}')
	RUSTFLAGS='--cfg cortexm_core="cortexm33f_r0p2" --cfg nrf_mcu="nrf9160"' cargo test --package drone-nrf-map --features "{{features}} std" --target=$(rustc --version --verbose | sed -n '/host/{s/.*: //;p}')

# Update README.md
readme:
	cargo readme -o README.md

# Bump the versions
version-bump version drone-core-version drone-cortexm-version drone-svd-version:
	sed -i "s/\(api\.drone-os\.com\/drone-nrf-map\/\)[0-9]\+\(\.[0-9]\+\)\+/\1$(echo {{version}} | sed 's/\(.*\)\.[0-9]\+/\1/')/" \
		Cargo.toml src/pieces/*/Cargo.toml src/pieces/Cargo.toml src/periph/*/Cargo.toml svd/Cargo.toml src/lib.rs
	sed -i '/\[.*\]/h;/version = ".*"/{x;s/\[package\]/version = "{{version}}"/;t;x}' \
		Cargo.toml src/pieces/*/Cargo.toml src/pieces/Cargo.toml src/periph/*/Cargo.toml svd/Cargo.toml
	sed -i '/\[.*\]/h;/version = "=.*"/{x;s/\[.*drone-nrf-map-.*\]/version = "={{version}}"/;t;x}' \
		Cargo.toml src/pieces/*/Cargo.toml src/pieces/Cargo.toml src/periph/*/Cargo.toml
	sed -i '/\[.*\]/h;/version = ".*"/{x;s/\[.*drone-core\]/version = "{{drone-core-version}}"/;t;x}' \
		Cargo.toml src/pieces/*/Cargo.toml src/pieces/Cargo.toml src/periph/*/Cargo.toml
	sed -i '/\[.*\]/h;/version = ".*"/{x;s/\[.*drone-cortexm\]/version = "{{drone-cortexm-version}}"/;t;x}' \
		Cargo.toml src/pieces/*/Cargo.toml src/pieces/Cargo.toml src/periph/*/Cargo.toml
	sed -i '/\[.*\]/h;/version = ".*"/{x;s/\[.*drone-svd\]/version = "{{drone-svd-version}}"/;t;x}' \
		svd/Cargo.toml
	sed -i 's/\(drone-nrf-map.*\)version = "[^"]\+"/\1version = "{{version}}"/' \
		src/lib.rs

# Publish to crates.io
publish:
	cd svd && cargo publish \
		--target=$(rustc --version --verbose | sed -n '/host/{s/.*: //;p}')
	sleep 30
	cd src/pieces/1 && cargo publish
	cd src/pieces/2 && cargo publish
	cd src/pieces/3 && cargo publish
	cd src/pieces/4 && cargo publish
	cd src/pieces/5 && cargo publish
	cd src/pieces/6 && cargo publish
	cd src/pieces/7 && cargo publish
	cd src/pieces/8 && cargo publish
	cd src/pieces/9 && cargo publish
	cd src/pieces/10 && cargo publish
	cd src/pieces/11 && cargo publish
	cd src/pieces/12 && cargo publish
	sleep 30
	cd src/pieces && cargo publish
	sleep 30
	cd src/periph/uarte && cargo publish
	sleep 30
	cargo publish --features "{{features}}"

# Publish the docs to api.drone-os.com
publish-doc: doc
	dir=$(sed -n 's/.*api\.drone-os\.com\/\(.*\/.*\)\/.*\/"/\1/;T;p' Cargo.toml) \
		&& rm -rf ../drone-api/$dir \
		&& cp -rT target/doc ../drone-api/$dir \
		&& cp -rT target/{{target}}/doc ../drone-api/$dir \
		&& echo '<!DOCTYPE html><meta http-equiv="refresh" content="0; URL=./drone_nrf_map">' > ../drone-api/$dir/index.html \
		&& cd ../drone-api && git add $dir && git commit -m "Docs for $dir"
