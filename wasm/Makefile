.PHONY: build-wasm build-deno build serve fmt

build-wasm:
	wasm-pack build -t web -d ./web/wasm

build-deno:
	deno task -c ./web/deno.json build

build: build-wasm build-deno
	cp ./web/wasm/wasm_bg.wasm ../docs/wasm_bg.wasm

serve:
	cd ../docs; python3 -m http.server

fmt:
	cargo fmt
	deno fmt -c ./web/deno.json
