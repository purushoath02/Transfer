
# File Transfer CLI Tool

A CLI tool for sending and receiving files over TCP, built using Rust and Tokio. This tool allows you to send files to a remote address or receive files from a remote sender.

## Features

- **Send Files**: Transfer files to a specified address.
- **Receive Files**: Listen for incoming file transfers and save them to a specified directory.

## Installation

1. **Install Rust**: Ensure you have Rust installed on your machine. If not, you can install it via [rustup](https://rustup.rs/):

    ```sh
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

2. **Clone the Repository**:

    ```sh
    git clone https://github.com/purushoath02/Transfer.git
    cd Transfer
    ```

3. **Build the Project**:

    ```sh
    cargo build --release
    ```

   This will create an executable in the `target/release` directory.

## Usage
### Create Reciever
Run the command
```sh
./target/release/Transfer receive
```
You will get the **address** of receiver.

### Sending a File

To send the file to the receiver, use the following command:

```sh
./target/release/Transfer send <file_path> <address>
```


## Code Overview

- main.rs: Contains the CLI logic, including argument parsing and handling file sending and receiving modes.
- utils.rs: Contains utility functions for creating connections, sharing files, and receiving files.

## Dependencies

- tokio: Asynchronous runtime for Rust.
- clap: Command-line argument parser.
- regex: Regular expressions for filename extraction.
- serde & bincode: For serializing and deserializing file data.
