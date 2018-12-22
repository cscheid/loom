## A Rust Raytracer based on Pete Shirley's "Ray Tracing minibooks"

Seriously, just go buy these books right now, they're U$3.00 each and
worth 10x as much, easily.

# Compiling

This should work:

	cargo build --release
	
# Usage

Minimal example: (you'll need python3 and
[`click`](https://pymbook.readthedocs.io/en/latest/click.html)
available in your environment)

    ./scripts/run.py --scene tests/s1.json --output out.png

Get more options with `./scripts/run.py --help`



  
