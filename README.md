### Set the desktops wallpaper of your mac from a terminal

Can be compiled with the latest version of the rustc compiler available [here](http://static.rust-lang.org/dist/rust-nightly-x86_64-apple-darwin.pkg).

To compile you just need to:

```shell
git clone https://github.com/gaku-sei/set-wallpaper.git
cd set-wallpaper
make
```

The binary is available in the target directory.

### Usage

```shell
# With a simple path or url:
/path/to/bin/set-wallpaper [path/to/picture|url]
# You can also use the -k or --kill option to automatically kill the dock and relaunch it:
/path/to/bin/set-wallpaper -k [path/to/picture|url]
# Additionally you can get help with:
/path/to/bin/set-wallpaper -h
```
