extern crate sqlite3;

use std::os;
use url::Url;
use std::io::{IoResult, TcpStream, File};
use sqlite3::{SqliteResult};

pub fn update_desktop_db(path_to_img: &Path) -> SqliteResult<()> {
  //unwrap, unwrap, unwrap... consider using match
  let path_to_db = os::homedir().unwrap().join_many(["Library", "Application Support", "Dock", "desktoppicture.db"]);
  let db = try!(sqlite3::open(path_to_db.as_str().unwrap()));
  let query = format!("UPDATE data SET value='{}';", path_to_img.as_str().unwrap());
  try!(db.exec(query.as_slice()));
  Ok(())
}

pub fn download_file(url: &Url) -> IoResult<Box<Path>> {
  let mut socket = try!(TcpStream::connect(url.host.as_slice(), 80));
  let req = format!(
    "GET {:s} HTTP/1.1\r\nHost: {:s}\r\nAccept: */*\r\n\r\n",
    url.path.path.as_slice(), url.host
  );
  try!(socket.write(req.as_bytes()));
  let res = try!(socket.read_to_end());

  //Find the good folder and picture name here:
  let filepath = box os::homedir().unwrap().join_many(["Pictures", "foobar"]);
  //let filepath = box os::tmpdir().join(Path::new("foobar"));
  let mut file = File::create(filepath);
  let res_without_headers = without_headers(&res);
  try!(file.write(res_without_headers.as_slice()));
  Ok(filepath)
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
