all:
	mkdir ./target
	rustc -o ./target/set-wallpaper -L ./include main.rs

clean:
	rm -r target
