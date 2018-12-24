all: release

debug:
	cargo build

release:
	cargo build --release

dist:
	tar cvzf loom-dist.tar.gz scripts ec2-driver target/release/loom-render

