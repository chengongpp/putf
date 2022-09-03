use std::{
    io::{Read, Write},
    net::TcpListener,
    path, thread, sync::Arc,
};
use clap::Parser;

const MAX_FILE_SIZE: u64 = 100_000_000;


#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    #[clap(short = 'l', long)]
    listen: String,
    #[clap(short = 'f', long)]
    file: String,
}
fn main() {
    let args = Args::parse();
    let listener = TcpListener::bind(args.listen.clone());
    let listener = match listener {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!("Failed to bind to {}: {}", args.listen, e);
            std::process::exit(1);
        }
    };

    let filesize = std::fs::metadata(args.file.clone());
    let filesize = match filesize {
        Ok(filesize) => filesize.len(),
        Err(e) => {
            eprintln!("Failed to open {}: {}", args.file, e);
            std::process::exit(1);
        }
    };
    if filesize > MAX_FILE_SIZE {
        eprintln!("Failed to open {}: File too large", args.file);
        std::process::exit(1);
    }
    let file = std::fs::File::open(args.file.clone());
    let mut file = match file {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to open {}: {}", args.file, e);
            std::process::exit(1);
        }
    };
    let mut buffer = vec![0; filesize as usize];
    let sz = file.read(&mut buffer);
    let sz = match sz {
        Ok(sz) => sz,
        Err(e) => {
            eprintln!("Failed to read {}: {}", args.file, e);
            std::process::exit(1);
        }
    };
    if sz != filesize as usize {
        eprintln!("Failed to read {}: File size changed", args.file);
        std::process::exit(1);
    }
    let ro_buffer_ptr = Arc::new(buffer);

    println!("Listening at {}", listener.local_addr().unwrap());
    println!("Copy the following command to receive file through bash");
    let filename = Arc::new(args.file.split(path::MAIN_SEPARATOR).last().unwrap());
    println!(
        "cat < /dev/tcp/{}/{} > {}",
        listener.local_addr().unwrap().ip(),
        listener.local_addr().unwrap().port(),
        filename
    );
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let ro_buffer_ptr_local = ro_buffer_ptr.clone();
        thread::spawn(move || {
            println!("New connection from {}", stream.peer_addr().unwrap());
            let write_result = stream.write_all(&ro_buffer_ptr_local);
            match write_result {
                Ok(_) => println!("File sent to {}. Don't forget to md5sum", stream.peer_addr().unwrap()),
                Err(e) => eprintln!("Failed to send file to {}: {}", stream.peer_addr().unwrap(), e),
            }
        });
    }
}
