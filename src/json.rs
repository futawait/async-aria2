use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Request {
    pub jsonrpc: String,
    pub id: String,
    pub method: String,
    pub params: Params,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum Params {
    V(Vec<String>),
    SV(String, Vec<String>),
    SSV(String, String, Vec<String>),
    SOptions(String, Options),
    SUU(String, usize, usize),
}

#[derive(Serialize)]
pub struct Options {
    #[serde(rename = "max-overall-download-limit")]
    pub max_overall_download_limit: String,
    #[serde(rename = "max-overall-upload-limit")]
    pub max_overall_upload_limit: String,
    pub dir: String,
}

#[derive(Deserialize)]
pub struct Response {
    pub jsonrpc: String,
    pub id: String,
    pub result: JsonResult,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum JsonResult {
    S(String),
    Status(Status),
    StatusList(Vec<Status>),
    Statistics(Statistics),
}

#[derive(Deserialize, Debug)]
pub struct Status {
    pub bitfield: String,
    pub bittorrent: Option<Bittorrent>,
    #[serde(rename = "completedLength")]
    pub completed_length: String,
    pub connections: String,
    pub dir: String,
    #[serde(rename = "downloadSpeed")]
    pub download_speed: String,
    pub files: Vec<File>,
    pub gid: String,
    #[serde(rename = "infoHash")]
    pub info_hash: Option<String>,
    #[serde(rename = "numPieces")]
    pub num_pieces: String,
    #[serde(rename = "numSeeders")]
    pub num_seeders: Option<String>,
    #[serde(rename = "pieceLength")]
    pub piece_length: String,
    pub seeder: Option<String>,
    pub status: String,
    #[serde(rename = "totalLength")]
    pub total_length: String,
    #[serde(rename = "uploadLength")]
    pub upload_length: String,
    #[serde(rename = "uploadSpeed")]
    pub upload_speed: String,
}

#[derive(Deserialize, Debug)]
pub struct Bittorrent {
    #[serde(rename = "announceList")]
    pub announce_list: Vec<Vec<String>>,
    pub comment: String,
    #[serde(rename = "creationDate")]
    pub creation_date: u32,
    pub info: Info,
    pub mode: String,
}

#[derive(Deserialize, Debug)]
pub struct Info {
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct File {
    #[serde(rename = "completedLength")]
    pub completed_length: String,
    pub index: String,
    pub length: String,
    pub path: String,
    pub selected: String,
    pub uris: Vec<Uri>,
}

#[derive(Deserialize, Debug)]
pub struct Uri {
    pub status: String,
    pub uri: String,
}

#[derive(Deserialize, Debug)]
pub struct Statistics {
    #[serde(rename = "downloadSpeed")]
    pub download_speed: String,
    #[serde(rename = "numActive")]
    pub num_active: String,
    #[serde(rename = "numStopped")]
    pub num_stopped: String,
    #[serde(rename = "numStoppedTotal")]
    pub num_stopped_total: String,
    #[serde(rename = "numWaiting")]
    pub num_waiting: String,
    #[serde(rename = "uploadSpeed")]
    pub upload_speed: String,
}
