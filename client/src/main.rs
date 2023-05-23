use std::env;
use std::fs::File;
use std::io::{self,Read, Write};
use std::net::{TcpListener,TcpStream};
use flate2::write::GzEncoder;
use flate2::Compression;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Uso: emisor <archivo> <ip_destino:puerto_destino>");
        return Ok(());
    }
    let file_path = &args[1];
    let dest_address = &args[2];

    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    
    let mut parts = dest_address.split(':');
    let dest_ip = parts.next().unwrap();
    let dest_port: u16 = parts.next().unwrap().parse().unwrap();
    let mut socket = TcpStream::connect((dest_ip, dest_port))?;
    println!("Conectado a {}", socket.peer_addr()?);

    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&buffer)?;
    let compressed_data = encoder.finish()?;

    // Enviar el archivo en fragmentos
    for chunk in compressed_data.chunks(250 * 1024) {
        socket.write_all(chunk)?;
    }

    println!("Archivo enviado correctamente");

    Ok(())
}
