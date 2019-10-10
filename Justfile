build_target := 'thumbv7em-none-eabihf'
features := 'nrf52832'

# Install dependencies
deps:
	rustup target add {{build_target}}
	rustup component add clippy
	rustup component add rustfmt
	rustup component add rls rust-analysis rust-src
	type cargo-readme >/dev/null || cargo +stable install cargo-readme

# Reformat the source code
fmt:
	cargo fmt

# Check for mistakes
lint:
	cargo clippy --package drone-nrf-map-svd
	cargo clippy --target {{build_target}} --features "{{features}}" --all --exclude drone-nrf-map-svd

# Check each feature
check-all:
	rustup target add thumbv7m-none-eabi
	rustup target add thumbv7em-none-eabihf
	cargo check --package drone-nrf-map --features nrf52810 --target thumbv7em-none-eabihf
	cargo check --package drone-nrf-map --features nrf52811 --target thumbv7em-none-eabihf
	cargo check --package drone-nrf-map --features nrf52832 --target thumbv7em-none-eabihf
	cargo check --package drone-nrf-map --features nrf52840 --target thumbv7em-none-eabihf

# Generate the docs
doc:
	cargo doc --package drone-nrf-map-svd
	cargo doc --target {{build_target}} --features "{{features}}" --package drone-nrf-map

# Open the docs in a browser
doc-open: doc
	cargo doc --target {{build_target}} --features "{{features}}" --package drone-nrf-map --open

# Update README.md
readme:
	cargo readme -o README.md

# Bump crate versions
version-bump version drone-core-version drone-cortex-m-version drone-cortex-m-svd-version:
	sed -i "s/\(api\.drone-os\.com\/drone-nrf-map\/\)[0-9]\+\(\.[0-9]\+\)\+/\1$(echo {{version}} | sed 's/\(.*\)\.[0-9]\+/\1/')/" \
		Cargo.toml src/pieces/*/Cargo.toml src/pieces/Cargo.toml svd/Cargo.toml src/lib.rs
	sed -i '/\[.*\]/h;/version = ".*"/{x;s/\[package\]/version = "{{version}}"/;t;x}' \
		Cargo.toml src/pieces/*/Cargo.toml src/pieces/Cargo.toml svd/Cargo.toml
	sed -i '/\[.*\]/h;/version = "=.*"/{x;s/\[.*drone-nrf-map-.*\]/version = "={{version}}"/;t;x}' \
		Cargo.toml src/pieces/*/Cargo.toml src/pieces/Cargo.toml
	sed -i '/\[.*\]/h;/version = ".*"/{x;s/\[.*drone-core\]/version = "{{drone-core-version}}"/;t;x}' \
		Cargo.toml src/pieces/*/Cargo.toml src/pieces/Cargo.toml
	sed -i '/\[.*\]/h;/version = ".*"/{x;s/\[.*drone-cortex-m\]/version = "{{drone-cortex-m-version}}"/;t;x}' \
		Cargo.toml src/pieces/*/Cargo.toml src/pieces/Cargo.toml
	sed -i '/\[.*\]/h;/version = ".*"/{x;s/\[.*drone-cortex-m-svd\]/version = "{{drone-cortex-m-svd-version}}"/;t;x}' \
		svd/Cargo.toml
	sed -i 's/\(drone-nrf-map.*\)version = "[^"]\+"/\1version = "{{version}}"/' \
		src/lib.rs

# Publish to crates.io
publish:
	cd svd && cargo publish
	sleep 5
	cd src/pieces/1 && cargo publish --target {{build_target}} --features "{{features}}"
	sleep 5
	cd src/pieces/2 && cargo publish --target {{build_target}} --features "{{features}}"
	sleep 5
	cd src/pieces/3 && cargo publish --target {{build_target}} --features "{{features}}"
	sleep 5
	cd src/pieces/4 && cargo publish --target {{build_target}} --features "{{features}}"
	sleep 5
	cd src/pieces/5 && cargo publish --target {{build_target}} --features "{{features}}"
	sleep 5
	cd src/pieces/6 && cargo publish --target {{build_target}} --features "{{features}}"
	sleep 5
	cd src/pieces/7 && cargo publish --target {{build_target}} --features "{{features}}"
	sleep 5
	cd src/pieces/8 && cargo publish --target {{build_target}} --features "{{features}}"
	sleep 5
	cd src/pieces/9 && cargo publish --target {{build_target}} --features "{{features}}"
	sleep 5
	cd src/pieces/10 && cargo publish --target {{build_target}} --features "{{features}}"
	sleep 5
	cd src/pieces/11 && cargo publish --target {{build_target}} --features "{{features}}"
	sleep 5
	cd src/pieces/12 && cargo publish --target {{build_target}} --features "{{features}}"
	sleep 5
	cd src/pieces && cargo publish --target {{build_target}} --features "{{features}}"
	sleep 5
	cargo publish --target {{build_target}} --features "{{features}}"

# Publish the docs to api.drone-os.com
publish-doc: doc
	dir=$(sed -n 's/.*api\.drone-os\.com\/\(.*\)"/\1/;T;p' Cargo.toml) \
		&& rm -rf ../drone-api/$dir \
		&& cp -rT target/doc ../drone-api/$dir \
		&& cp -rT target/{{build_target}}/doc ../drone-api/$dir \
		&& echo '<!DOCTYPE html><meta http-equiv="refresh" content="0; URL=./drone_nrf_map">' > ../drone-api/$dir/index.html \
		&& cd ../drone-api && git add $dir && git commit -m "Docs for $dir"
