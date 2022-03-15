use std::net::{IpAddr, SocketAddr};
use serde::{Deserialize};
use crate::{InfoHash, PeerId};
use crate::torrust_http_tracker::Bytes;

#[derive(Deserialize)]
pub struct AnnounceRequestQuery {
    pub downloaded: Option<Bytes>,
    pub uploaded: Option<Bytes>,
    pub key: Option<String>,
    pub port: u16,
    pub left: Option<Bytes>,
    pub event: Option<String>,
    pub compact: Option<u8>,
}

#[derive(Debug)]
pub struct AnnounceRequest {
    pub info_hash: InfoHash,
    pub peer_addr: SocketAddr,
    pub forwarded_ip: Option<IpAddr>,
    pub downloaded: Bytes,
    pub uploaded: Bytes,
    pub peer_id: PeerId,
    pub port: u16,
    pub left: Bytes,
    pub event: Option<String>,
    pub compact: Option<u8>,
}

pub struct ScrapeRequest {
    pub info_hashes: Vec<InfoHash>,
}
