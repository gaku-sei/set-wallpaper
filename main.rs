#![feature(phase)]
#[phase(plugin)]
extern crate regex_macros;
extern crate regex;
extern crate url;
extern crate getopts;

extern crate sqlite3;

use std::os;
use url::Url;
use getopts::{OptGroup, optflag, getopts};
use sqlite3::{SqliteResult};

mod io;

fn exec(arg: &String) -> SqliteResult<()> {
    let re = regex!(r"http://|https://");
    //Get the path to the downloaded file or take the provided path
    let path =
        if re.is_match(arg.as_slice()) {
            match Url::parse(arg.as_slice()) {
                //Try to download file
                Ok(url) => match io::download_file(&url) {
                    Ok(p) => p,
                    Err(e) => fail!("Io Error: {}", e)
                },
                Err(e) => fail!("Url Error: {}", e)
            }
        } else {
            Path::new(arg.as_slice())
        };
    Ok(try!(io::update_desktop_db(&path)))
}

//Print help message
fn print_usage(program: &String, opts: &[OptGroup]) {
    println!("Usage: {} [/path/to/picture|url]", program);
    for opt in opts.iter() {
        println!("-{} --{}\t{}", opt.short_name, opt.long_name, opt.desc);
    }
}

//Clean && Deal with errors
#[allow(unused_must_use)]
fn main() {
    //Reading arguments and declarting options
    let raw_args = os::args();
    let program = raw_args.get(0).clone();

    let opts = [
        optflag("k", "kill", "refresh the ui with `killall dock`"),
        optflag("h", "help", "print this help menu")
    ];

    //Extraction arguments
    let (matches, args) = match getopts(raw_args.tail(), opts) {
        Ok(m) => (m.clone(), m.free),
        Err(f) => fail!(f.to_string())
    };

    //Display help message if -h flag is provided or if there is no argument
    if matches.opt_present("h") || args.len() == 0 {
        print_usage(&program, opts);
        return;
    }

    //Main function call
    if args.len() > 0 {
        exec(args.get(0));
        //Kill dock if -k flag is provided
        if matches.opt_present("k") {
            io::kill_dock();
        }
    }
}
