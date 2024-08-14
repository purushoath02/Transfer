use regex::Regex;
use serde::{Deserialize, Serialize};
use std::io;
use std::path::Path;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[derive(Serialize, Deserialize, Debug)]
struct FileFrame {
    name: String,
    content: Vec<u8>,
}

pub async fn create_connection(address: &str) -> std::io::Result<TcpStream> {
    let connection = TcpStream::connect(address).await?;
    Ok(connection)
}

pub async fn share_file(connection: &mut TcpStream, path: &str) -> std::io::Result<()> {
    let mut file = File::open(path).await?;
    let mut content = Vec::new();
    file.read_to_end(&mut content).await?;

    let re = Regex::new(r".*/(.*)$").unwrap();
    let filename = re
        .captures(path)
        .and_then(|caps| caps.get(1).map(|m| m.as_str()))
        .unwrap();

    let frame = FileFrame {
        name: String::from(filename),
        content,
    };

    let packet: Vec<u8> = match bincode::serialize(&frame) {
        Ok(serialized) => serialized,
        Err(e) => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Serialization error: {}", e),
            ));
        }
    };
    connection.write_all(&packet).await?;
    Ok(())
}

// pub async fn receive_mode(address: &str) -> std::io::Result<TcpListener> {
//     let connection = TcpListener::bind(address).await?;
//     println!("Listening to : {:?}", address);
//     Ok(connection)
// }

async fn create_file(path: &String, filename: &String) -> std::io::Result<File> {
    let dir_path = Path::new(&path);
    let mut file_name = filename.clone();
    let mut counter = 1;
    loop {
        let file_path = dir_path.join(&file_name);
        if !file_path.exists() {
            let file = File::create(file_path).await?;
            return Ok(file);
        }
        file_name = format!("{}({})", filename, counter);
        counter += 1;
    }
}
pub async fn get_file(connection: &mut TcpStream, path: &String) -> std::io::Result<()> {
    let mut buffer: Vec<u8> = Vec::new();
    connection.read_to_end(&mut buffer).await?;
    let fileframe: FileFrame = match bincode::deserialize(&buffer) {
        Ok(frame) => frame,
        Err(e) => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Deserialization error: {}", e),
            ));
        }
    };

    let mut file = create_file(&path, &fileframe.name).await?;
    file.write_all(&fileframe.content).await?;
    println!("File {} saved successfully", fileframe.name);
    Ok(())
}
