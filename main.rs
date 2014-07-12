#![feature(phase)]
#[phase(plugin)]
extern crate regex_macros;
extern crate regex;
extern crate url;

extern crate sqlite3;

use std::os;
use url::Url;
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

#[allow(unused_must_use)]
fn main() {
  let args = os::args();
  if args.len() > 1 {
    exec(args.get(1));
  }
}
/*fn main() {
  let args = os::args();
  if args.len() > 1 {
    match exec(args.get(1)) {
      Ok(..) => println!("Done"),
      Err(e) => fail!("Error: {}", e)
    }
  } else {
    fail!("You should give one argument.");
  }
}*/
