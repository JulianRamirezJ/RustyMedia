use std::fs::File;
use std::fs;
use std::io::{self,Read, Write};
use std::net::{TcpListener,TcpStream};
use std::thread;
use std::path::PathBuf;
use tar::Archive;
use flate2::read::GzDecoder;

fn handle_emisor(mut socket: TcpStream) -> io::Result<()> {
    let file_path = "tmp.tar.gz";
    let output_file_path = PathBuf::from(&file_path);
    let mut output_file = File::create(output_file_path.clone())?;
    println!("Recibiendo archivo: {}", file_path);

    loop {
        let mut buffer = vec![0; 250 * 1024];
        match socket.read(&mut buffer) {
            Ok(bytes_received) if bytes_received > 0 => {
                output_file.write_all(&buffer[..bytes_received])?;
            }
            Ok(_) => break,
            Err(_e) => break,
        }
    }
    let tar_file = File::open(file_path)?;
    let tar = GzDecoder::new(tar_file);
    let mut archive = Archive::new(tar);
    archive.unpack("./received")?;
    fs::remove_file(&file_path)?;

    println!("Archivo descomprimido correctamente: {}", file_path);
    Ok(())
}


fn main() -> io::Result<()> {
    let port: u16 = 12345; 
    let listener = TcpListener::bind(("127.0.0.1", port))?;
    println!("Receptor escuchando en {}", listener.local_addr()?);

    for stream in listener.incoming() {
        match stream {
            Ok(socket) => {
                thread::spawn(move || {
                    match handle_emisor(socket) {
                        Ok(_) => (),
                        Err(err) => println!("Error al procesar conexión: {}", err),
                    }
                });
            }
            Err(err) => {
                println!("Error al aceptar conexión entrante: {}", err);
            }
        }
    }
    
    Ok(())
}
