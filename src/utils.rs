pub fn read_torrent_file(path: &str) -> String {
    base64::encode(&std::fs::read(path).unwrap())
}
