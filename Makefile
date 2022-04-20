all: build build-example
build:
	cargo build
build-example:
	cd examples/ && cargo build
buf-clean:
	find src/gen -type f \( -iname "*.rs" ! -iname "mod.rs" \) -delete
buf:	buf-clean
	buf generate buf.build/viamrobotics/goutils --template buf.gen.yaml
	buf generate buf.build/googleapis/googleapis --template buf.gen.yaml --path google/rpc --path google/api
	buf generate buf.build/viamrobotics/rdk --template buf.gen.yaml
