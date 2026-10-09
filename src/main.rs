use std::env;
use serde::{Deserialize, Serialize};
use std::fs;
use sha1::{Sha1, Digest};

#[derive(Deserialize)]
struct Torrent {
    announce: String,
    info: Info,
}
#[allow(dead_code)]
#[derive(Deserialize, Serialize)]
struct Info {
    name: String,
    length: usize,
    #[serde(rename = "piece length")]
    piece_length: usize,
    #[serde(with = "serde_bytes")]
    pieces: Vec<u8>,
}

#[derive(Deserialize, Serialize)]
struct TrackerResponse {
    interval: usize,
    #[serde(with = "serde_bytes")]
    peers: Vec<u8>
}

fn parse_torrent(arg: Vec<String>) -> anyhow::Result<(Torrent, String)> {
    let contents = fs::read(&arg[2])?;
    let deserialized: Torrent = serde_bencode::from_bytes(&contents)?;
    let re_encoded = serde_bencode::to_bytes(&deserialized.info)?;
    let hash = Sha1::digest(&re_encoded);
    let info_hash: String = hex::encode(hash);

    Ok((deserialized, info_hash))
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    match command.as_str() {
        "decode" => {
            eprintln!("Logs from your program will appear here!");
    
             let encoded_value = &args[2];
             let decoded_value: serde_bencode::value::Value = serde_bencode::from_str(encoded_value)?;
             println!("{:?}", decoded_value);
        },
        "info" => {

            let (deserialized, info_hash) = parse_torrent(args)?;            
            println!("Here is Tracker URL: {}", deserialized.announce);
            println!("Here is the length: {}", deserialized.info.length);
            println!("Here is the info hash: {}", info_hash);
        },
        "peers" => {
            let (deserialized, _info_hash) = parse_torrent(args)?;            
            let query = "peer_id=00112233445566778899&port=6881&uploaded=0&downloaded=0&left=92063&compact=1";
            let req = format!(
                "{}?info_hash={}&{}", 
                deserialized.announce,
                "%d6%9f%91%e6%b2%ae%4c%54%24%68%d1%07%3a%71%d4%ea%13%87%9a%7f", 
                query
            );

            let res = reqwest::blocking::get(&req)?;
            let body = res.bytes()?;
            let deserialized_res: TrackerResponse = serde_bencode::from_bytes(&body)?;

            println!("Peers: {:?}", deserialized_res.peers);
        },
        _ =>  println!("unknown command: {}", args[1])
    }

    Ok(())
}
