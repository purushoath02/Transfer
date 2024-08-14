use clap::{Arg, Command};
use tokio::net::TcpListener;
mod utils;
use std::env;
use std::net::SocketAddr;
use utils::{create_connection, get_file, share_file};

#[tokio::main]
async fn main() {
    let mut mode = String::new();
    let mut file_path = String::new();
    let mut address = String::new();

    let mut save_path = env::current_dir().unwrap().display().to_string();
    let matches = Command::new("CLI")
        .version("1.0")
        .author("Your Name")
        .about("CLI tool for sending and receiving files")
        .subcommand(
            Command::new("send")
                .about("Send file to a given address")
                .arg(
                    Arg::new("file")
                        .required(true)
                        .help("File to Send")
                        .index(1),
                )
                .arg(
                    Arg::new("address")
                        .required(true)
                        .help("Address of receiver")
                        .index(2),
                ),
        )
        .subcommand(
            Command::new("receive")
                .about("Receive file")
                .arg(Arg::new("path").required(false).help("Save Path").index(1)),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("send", send_matches)) => {
            file_path = send_matches.get_one::<String>("file").unwrap().to_string();
            address = send_matches
                .get_one::<String>("address")
                .unwrap()
                .to_string();

            println!("Sending file '{}' to address '{}'", file_path, address);
            mode = String::from("send");
            // Add your file sending logic here
        }
        Some(("receive", receive_matches)) => {
            mode = String::from("receive");
            let temp_save_path = receive_matches.get_one::<String>("path");
            match temp_save_path {
                Some(temp_save_path) => {
                    println!("Receiving file and saving to '{}'", temp_save_path);

                    save_path = temp_save_path.to_string().clone();
                }
                None => println!("Receiving file with no specific save path"),
            }
        }
        _ => {
            println!("No subcommand was used or unknown subcommand.");
        }
    };
    //println!("{:?}", mode);
    match mode.as_str() {
        "send" => match create_connection(address.as_str()).await {
            Ok(mut stream) => {
                println!("Connected to {:?}", stream.peer_addr().unwrap());

                share_file(&mut stream, &file_path).await.unwrap();
            }
            Err(e) => eprintln!("Failed to connect: {:?}", e),
        },
        "receive" => {
            let address = "0.0.0.0:8080"
                .parse::<SocketAddr>()
                .expect("Invalid address");

            let listener = TcpListener::bind(address).await.expect("error hosting");
            println!("listening to {:}", address.to_string());
            loop {
                match listener.accept().await {
                    Ok((mut connection, remote_addr)) => {
                        println!("Accepted connection from: {}", remote_addr);

                        let save_path_clone = save_path.clone();

                        tokio::spawn(async move {
                            if let Err(e) = get_file(&mut connection, &save_path_clone).await {
                                eprintln!("Error handling connection: {:?}", e);
                            }
                        });
                    }
                    Err(e) => {
                        eprintln!("Failed to accept connection: {:?}", e);
                    }
                }
            }
        }
        _ => {
            eprintln!("Unknown mode: {}", mode);
        }
    }
}
