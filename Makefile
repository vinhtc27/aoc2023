info:
	dust && tokei

unuse:
	cargo +nightly udeps --all-targets

bench:
	cargo +nightly bench

run:
	cargo run

test:
	cargo test --package aoc2023 --lib -- days::day01::tests::test_example_answers

update:
	rustup update && cargo outdated && cargo update && cargo upgrade && cargo audit

clean:
	cargo sweep --time 7 && cargo sweep --toolchains nightly-aarch64-apple-darwin && cargo sweep --installed
