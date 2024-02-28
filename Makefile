.PHONY: py-hello-http

py-hello-http: target/release/wasmtime-test gen/hello-http.py.wasm
	./target/release/wasmtime-test hello-http gen/hello-http.py.wasm

target/release/wasmtime-test: src/main.rs wit/hello-http.wit
	cargo build --release

hello_http/__init__.py: wit/hello-http.wit
	componentize-py -d wit -w hello-http bindings .

gen/hello-http.py.wasm: hello_http_app.py hello_http/__init__.py hello_requests.py hello_poll_loop.py
	mkdir -p gen
	componentize-py -d wit -w hello-http componentize hello_http_app -o gen/hello-http.py.wasm
