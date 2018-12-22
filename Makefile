all: release

debug:
	cargo build

release:
	cargo build --release
