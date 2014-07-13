extern crate sqlite3;
extern crate time;

use std::os;
use url::Url;
use std::io::{IoResult, TcpStream, File, Command, UserRWX};
use std::io::fs;
use sqlite3::{SqliteResult};

//Kill the dock the hard way
pub fn kill_dock() -> IoResult<()> {
    try!(Command::new("killall").arg("Dock").spawn());
    Ok(())
}

//Write the new wallpaper path to the db
pub fn update_desktop_db(path_to_img: &Path) -> SqliteResult<()> {
    //The path to db is ~/Library/Application Support/Dock/desktoppicture.db
    let path_to_db = os::homedir().unwrap().join_many(["Library", "Application Support", "Dock", "desktoppicture.db"]);
    //Open the db and set the query
    let db = try!(sqlite3::open(path_to_db.as_str().unwrap()));
    let query = format!("UPDATE data SET value='{}';", path_to_img.as_str().unwrap());
    //Make the update
    try!(db.exec(query.as_slice()));
    Ok(())
}

pub fn download_file(url: &Url) -> IoResult<Path> {
    //Open the socket to the host and set the req
    let mut socket = try!(TcpStream::connect(url.host.as_slice(), 80));
    let req = format!(
        "GET {:s} HTTP/1.1\r\nHost: {:s}\r\nAccept: */*\r\n\r\n",
        url.path.path.as_slice(), url.host
    );
    //Send request to the socket
    try!(socket.write(req.as_bytes()));
    //Read response from server and remove the header
    let res_without_headers = without_headers(&try!(socket.read_to_end()));
    //Write response to a new file and return the path
    write_to_file(res_without_headers.as_slice())
}

fn write_to_file(data: &[u8]) -> IoResult<Path> {
    //The path to the new picture is ~/Library/wallpaper/wall-{sec}-{nsec}
    let dirpath = os::homedir().unwrap().join_many(["Library", "wallpaper"]);
    //If the directory does not exist, create it
    if !dirpath.exists() {
        try!(fs::mkdir(&dirpath, UserRWX));
    }
    //Format the new file name dynamically
    let ts = time::now().to_timespec();
    let filepath = dirpath.join(format!("wall-{:d}-{:d}", ts.sec, ts.nsec));
    //Create file and write data
    let mut file = File::create(&filepath);
    try!(file.write(data));
    //Return the path
    Ok(filepath)
}

fn without_headers(vec: &Vec<u8>) -> Vec<u8> {
    //Define a new vec and the accumulator
    let mut new_vec = Vec::new();
    let mut acc = 0u;
    for b in vec.iter() {
        //If the accumulator is equal to 4, that is if \r\n\r\n has been met, push in the new vector
        if acc == 4 {
            new_vec.push(*b);
        } else {
            //Increment accumulator if \r or \n are met
            if *b == b'\r' || *b == b'\n' { acc += 1 } else { acc = 0 }
        }
    }
    new_vec
}
