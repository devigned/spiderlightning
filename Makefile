.PHONY: build
build:
	cargo build --release
	cargo build --target wasm32-wasi --release --manifest-path ./examples/multi_capability-demo/Cargo.toml
	cargo build --target wasm32-wasi --release --manifest-path ./examples/kv-demo/Cargo.toml
	cargo build --target wasm32-wasi --release --manifest-path ./examples/mq-sender-demo/Cargo.toml
	cargo build --target wasm32-wasi --release --manifest-path ./examples/mq-receiver-demo/Cargo.toml
	cargo build --target wasm32-wasi --release --manifest-path ./examples/lockd-demo/Cargo.toml
	cargo build --target wasm32-wasi --release --manifest-path ./examples/pubsub-producer-demo/Cargo.toml
	cargo build --target wasm32-wasi --release --manifest-path ./examples/pubsub-consumer-demo/Cargo.toml

build-app:
	cargo build --target wasm32-wasi --release --manifest-path ./examples/chat-app-sender-demo/Cargo.toml
	cargo build --target wasm32-wasi --release --manifest-path ./examples/chat-app-receiver-demo/Cargo.toml
	
.PHONY: test
test:
	cargo test --all --no-fail-fast -- --nocapture

.PHONY: check
check:
	cargo clippy --all-targets --all-features -- -D warnings
	cargo fmt --all -- --check

.PHONY: run
run:
	./target/release/slight -m ./target/wasm32-wasi/release/multi_capability-demo.wasm -c './examples/multi_capability-demo/wc.toml'
	./target/release/slight -m ./target/wasm32-wasi/release/kv-demo.wasm -c './examples/kv-demo/filekv-wc.toml' & python ./examples/kv-demo/simulate.py
	# ./target/release/slight -m ./target/wasm32-wasi/release/kv-demo.wasm -c './examples/kv-demo/azblobkv-wc.toml'
	./target/release/slight -m ./target/wasm32-wasi/release/mq-sender-demo.wasm -c './examples/mq-sender-demo/filemq-wc.toml' &
	./target/release/slight -m ./target/wasm32-wasi/release/mq-receiver-demo.wasm -c './examples/mq-receiver-demo/filemq-wc.toml'
	./target/release/slight -m ./target/wasm32-wasi/release/mq-sender-demo.wasm -c './examples/mq-sender-demo/azsbusmq-wc.toml' &
	./target/release/slight -m ./target/wasm32-wasi/release/mq-receiver-demo.wasm -c './examples/mq-receiver-demo/azsbusmq-wc.toml'
	./target/release/slight -m ./target/wasm32-wasi/release/lockd-demo.wasm -c './examples/lockd-demo/wc.toml' &
	./target/release/slight -m ./target/wasm32-wasi/release/lockd-demo.wasm -c './examples/lockd-demo/wc.toml'
	./target/release/slight -m ./target/wasm32-wasi/release/pubsub-consumer-demo.wasm -c './examples/pubsub-consumer-demo/wc.toml' &
	./target/release/slight -m ./target/wasm32-wasi/release/pubsub-producer-demo.wasm -c './examples/pubsub-producer-demo/wc.toml'

run-c:
	./target/release/slight -m ./target/wasm32-wasi/release/mq-sender-demo.wasm -c './examples/mq-sender-demo/filemq-wc.toml' && ./target/release/slight -m ./examples/kv-mq-demo-clang/kv-mq-filesystem-c.wasm -c './examples/kv-mq-demo-clang/wc.toml'

run-sender-app:
	./target/release/slight -m ./target/wasm32-wasi/release/chat-app-sender-demo.wasm -c './examples/chat-app-sender-demo/slightfile.toml'

run-receiver-app:
	./target/release/slight -m ./target/wasm32-wasi/release/chat-app-receiver-demo.wasm -c './examples/chat-app-receiver-demo/slightfile.toml' main
