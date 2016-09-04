use super::create_transmission;
use std::io::{self, Write, Cursor};
use requests::AddTorrent;

#[test]
pub fn from_file() {
    let mut tr = create_transmission();

    let data = include_bytes!("./dummy.torrent");
    let mut torrent = Cursor::new(&data[..]);
    
    let req = AddTorrent::from_reader(&mut torrent).expect("Error while creating the request!");
    
    let res = tr.send(&req).expect("Error while communicating with the server!");
}
