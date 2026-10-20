use std::{env, io::{Read, Write}, net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpStream}, time::Duration};
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

fn parse_torrent(arg: Vec<String>) -> anyhow::Result<(Torrent, Vec<u8>, String)> {
    let contents = fs::read(&arg[2])?;
    let deserialized: Torrent = serde_bencode::from_bytes(&contents)?;
    let re_encoded = serde_bencode::to_bytes(&deserialized.info)?;
    let hash = Sha1::digest(&re_encoded);
    let info_hash_bytes = hash.to_vec();
    let info_hash: String = hex::encode(hash);

    Ok((deserialized, info_hash_bytes, info_hash))
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

            let (deserialized, _info_hash_bytes, info_hash) = parse_torrent(args)?;            
            println!("Here is Tracker URL: {}", deserialized.announce);
            println!("Here is the length: {}", deserialized.info.length);
            println!("Here is the info hash: {}", info_hash);
        },
        "peers" => {
            let (deserialized, info_hash_bytes, _info_hash) = parse_torrent(args)?;            
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

            let mut valid_peers: Vec<SocketAddr> = Vec::new();

            for address in deserialized_res.peers.chunks_exact(6) {
                let ip = Ipv4Addr::new(address[0], address[1], address[2], address[3]);
                let port = u16::from_be_bytes([address[4], address[5]]);
                let addr = SocketAddr::V4(SocketAddrV4::new(ip, port));

                match TcpStream::connect_timeout(&addr, Duration::from_secs(3)) {
                    Ok(mut stream) => {
                        let mut handshake = Vec::new();
                        handshake.push(19u8);
                        handshake.extend_from_slice(b"BitTorrent protocol");
                        handshake.extend_from_slice(&[0u8; 8]);
                        handshake.extend_from_slice(&info_hash_bytes);
                        handshake.extend_from_slice(b"00112233445566778899");
                        
                        if let Err(e) = stream.write_all(&handshake) {
                            eprintln!("{}: write failed: {}", addr, e);
                            continue;
                        }
                        
                         let mut buf = [0u8; 68];

                         if let Err(e) = stream.read_exact(&mut buf) {
                             eprintln!("{}: read failed: {}", addr, e);
                             continue;
                         }

                         let pstrlen = buf[0];                // offset byte, should be 19.
                         let pstr = &buf[1..20];              //  "BitTorrent protocol" banner       
                         let reply_info_hash = &buf[28..48];          
                         let reply_peer_id = &buf[48..68];    // peer's own id 

                         if  pstrlen != 19 {
                             eprintln!("{}: bad ptrstr: {}", addr, pstrlen);
                             continue;
                         }

                         if pstr != b"BitTorrent protocol" {
                            eprintln!("{}: bad protocol string", addr);
                            continue;
                         }

                         if reply_info_hash != info_hash_bytes.as_slice() {  // same info_hash should be echoed back.
                               eprintln!("{}: info_hash mismatch", addr);
                                 continue;
                             }
                         if reply_peer_id == b"00112233445566778899" {
                             eprintln!("{}: peer is ourselves", addr);
                             continue;
                            }
                         
                            println!("{} peer id: {}", addr, hex::encode(reply_peer_id));
                            valid_peers.push(addr); 
                    },
                    Err(_) => {
                        eprintln!("address {} failed", addr);
                    }
                
                }
                
            }

        },
        _ =>  println!("unknown command: {}", args[1])
    }

    Ok(())
}
