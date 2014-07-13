#![feature(phase)]
#[phase(plugin)]
extern crate regex_macros;
extern crate regex;
extern crate url;
extern crate getopts;

extern crate sqlite3;

use std::os;
use url::Url;
use getopts::{Matches, optflag, getopts};
use sqlite3::{SqliteResult};

mod io;

fn exec(arg: &String) -> SqliteResult<()> {
    let re = regex!(r"http://|https://");
    let path =
        if re.is_match(arg.as_slice()) {
            match Url::parse(arg.as_slice()) {
                Ok(url) => match io::download_file(&url) {
                    Ok(p) => p,
                    Err(e) => fail!("Io Error: {}", e)
                },
                Err(e) => fail!("Url Error: {}", e)
            }
        } else {
            box Path::new(arg.as_slice())
        };
    Ok(try!(io::update_desktop_db(path)))
}

fn extract_args() -> (Matches, Vec<String>) {
    let args = os::args();
    let opts = [ optflag("k", "kill", "Killall Dock") ];
    match getopts(args.tail(), opts) {
        Ok(m) => (m.clone(), m.free),
        Err(f) => fail!(f.to_string())
    }
}

#[allow(unused_must_use)]
fn main() {
    let (matches, args) = extract_args();
    if args.len() > 0 {
        exec(args.get(0));
        if matches.opt_present("k") {
            io::kill_dock();
        }
    }
}
