extern crate sqlite3;
extern crate time;

use std::os;
use url::Url;
use std::io::{IoResult, TcpStream, File, Command, UserRWX};
use std::io::fs;
use sqlite3::{SqliteResult};

pub fn kill_dock() -> IoResult<()> {
    try!(Command::new("killall").arg("Dock").spawn());
    Ok(())
}

pub fn update_desktop_db(path_to_img: &Path) -> SqliteResult<()> {
    let path_to_db = os::homedir().unwrap().join_many(["Library", "Application Support", "Dock", "desktoppicture.db"]);
    let db = try!(sqlite3::open(path_to_db.as_str().unwrap()));
    let query = format!("UPDATE data SET value='{}';", path_to_img.as_str().unwrap());
    try!(db.exec(query.as_slice()));
    Ok(())
}

//Split this function
pub fn download_file(url: &Url) -> IoResult<Path> {
    let mut socket = try!(TcpStream::connect(url.host.as_slice(), 80));
    let req = format!(
        "GET {:s} HTTP/1.1\r\nHost: {:s}\r\nAccept: */*\r\n\r\n",
        url.path.path.as_slice(), url.host
    );
    try!(socket.write(req.as_bytes()));
    let res = try!(socket.read_to_end());

    let dirpath = os::homedir().unwrap().join_many(["Library", "wallpaper"]);
    if !dirpath.exists() {
        try!(fs::mkdir(&dirpath, UserRWX));
    }
    let ts = time::now().to_timespec();
    let filepath = dirpath.join(format!("wall-{:d}-{:d}", ts.sec, ts.nsec));
    let mut file = File::create(&filepath);
    let res_without_headers = without_headers(&res);
    try!(file.write(res_without_headers.as_slice()));
    Ok(filepath)
}

fn without_headers(vec: &Vec<u8>) -> Vec<u8> {
    let mut new_vec = Vec::new();
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
