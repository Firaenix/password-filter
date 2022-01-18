install-tools:
	cargo install wasm-bindgen-cli --git https://github.com/bsvwasm/wasm-bindgen --branch universal-ui64-patch


check-format:
	cargo fmt -- --check && cargo clippy -- -Dwarnings

build-web:
	cargo build --target wasm32-unknown-unknown --release
	wasm-bindgen ./target/wasm32-unknown-unknown/release/password_filter.wasm --out-dir pkg/web --target web --reference-types --weak-refs

build-bundler:
	wasm-pack build --release --out-dir ./pkg/bundler --target bundler

build-nodejs:
	wasm-pack build --release --out-dir ./pkg/node --target nodejs

build-wasm:
	make build-web ; make build-bundler ; make build-nodejs

test-node:
	make build-nodejs && pushd ./examples/node-test && yarn test ; popd

publish-node:
	# make sure not to call make build-* because wasm-pack doesnt allow you to specify subdirectories.
	wasm-pack build --release --target nodejs
	gsed -i "0,/sha256_pow/s//sha256-pow/" ./pkg/package.json
	wasm-pack publish ./pkg

publish-bundler:
	wasm-pack build --release --target bundler
	gsed -i "0,/sha256_pow/s//sha256-pow-bundler/" ./pkg/package.json
	wasm-pack publish ./pkg