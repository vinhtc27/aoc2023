info:
	dust && tokei

unuse:
	cargo +nightly udeps --all-targets

bench:
	cargo +nightly bench

lint:
	cargo clippy

run:
	cargo run

test:
	make run TEST=1

update:
	rustup update && cargo outdated && cargo update && cargo upgrade && cargo audit

clean:
	cargo sweep --time 7 && cargo sweep --toolchains nightly-aarch64-apple-darwin && cargo sweep --installed
