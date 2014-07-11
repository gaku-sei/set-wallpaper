#![feature(phase)]
#[phase(plugin)]
extern crate regex_macros;
extern crate regex;
extern crate url;

extern crate sqlite3;

use std::os;
use std::io::{IoResult, TcpStream, File};
use url::Url;
use sqlite3::{SqliteResult};

fn update_desktop_db(path_to_img: &Path) -> SqliteResult<()> {
  let path_to_db = os::homedir().unwrap().join_many(["Library", "Application Support", "Dock", "desktoppicture.db"]);
  let db = try!(sqlite3::open(path_to_db.as_str().unwrap()));
  let query = format!("UPDATE data SET value='{}';", path_to_img.as_str().unwrap());
  try!(db.exec(query.as_slice()));
  Ok(())
}

fn without_headers(vec: &Vec<u8>) -> Box<Vec<u8>> {
  let mut new_vec = box Vec::new();
  let mut acc = 0u;
  for b in vec.iter() {
    if acc == 4 {
      new_vec.push(*b);
    } else {
      if *b == b'\n' || *b == b'\r' { acc += 1 } else { acc = 0 }
    }
  }
  new_vec
}

fn download_file(url: &Url) -> IoResult<Box<Path>> {
  let mut socket = try!(TcpStream::connect(url.host.as_slice(), 80));
  let req = format!(
    "GET {:s} HTTP/1.1\r\nHost: {:s}\r\nAccept: */*\r\n\r\n",
    url.path.path.as_slice(), url.host
  );
  try!(socket.write(req.as_bytes()));
  let res = try!(socket.read_to_end());

  let filepath = box os::homedir().unwrap().join_many(["Pictures", "foobar"]);
  //let filepath = box os::tmpdir().join(Path::new("foobar"));
  let mut file = File::create(filepath);
  let res_without_headers = without_headers(&res);
  try!(file.write(res_without_headers.as_slice()));
  Ok(filepath)
}

//use try!
fn exec(arg: &String) -> SqliteResult<()> {
  let re = regex!(r"http://|https://");
  let path =
    if re.is_match(arg.as_slice()) {
      match Url::parse(arg.as_slice()) {
        Ok(url) => match download_file(&url) {
          Ok(p) => p,
          Err(e) => fail!("Io Error: {}", e)
        },
        Err(e) => fail!("Url Error: {}", e)
      }
    } else {
      box Path::new(arg.as_slice())
    };
  Ok(try!(update_desktop_db(path)))
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
