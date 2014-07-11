all:
	mkdir ./target
	rustc --out-dir ./target -L ./include main.rs

clean:
	rm -r target
