use std::env;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct Torrent {
    announce: String,
    info: Info,
}
#[allow(dead_code)]
#[derive(Deserialize)]
struct Info {
    name: String,
    length: usize,
    #[serde(rename = "piece length")]
    piece_length: usize,
    #[serde(with = "serde_bytes")]
    pieces: Vec<u8>,
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
            let contents = fs::read(&args[2])?;
            let deserialized: Torrent = serde_bencode::from_bytes(&contents)?;

            println!("Here is Tracker URL: {}", deserialized.announce);
            println!("Here is the length: {}", deserialized.info.length);
            
        },
        _ =>  println!("unknown command: {}", args[1])
    }

    Ok(())
}
