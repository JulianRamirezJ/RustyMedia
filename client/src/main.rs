use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::net::TcpStream;
use tar::Builder;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs;
use std::path::{Path, PathBuf};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Uso: emisor <archivo> <ip_destino:puerto_destino>");
        return Ok(());
    }
    let file_path = &args[1];
    let dest_address = &args[2];

    match create_tarball(file_path) {
        Ok(output_file) => {
            println!("Archivo tar creado exitosamente: {}", output_file);
        }
        Err(err) => {
            eprintln!("Error al crear el archivo tar: {}", err);
        }
    }

    let tar_path = "tmp.tar.gz";
    let mut file = File::open(tar_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let mut parts = dest_address.split(':');
    let dest_ip = parts.next().unwrap();
    let dest_port: u16 = parts.next().unwrap().parse().unwrap();
    let mut socket = TcpStream::connect((dest_ip, dest_port))?;
    println!("Conectado a {}", socket.peer_addr()?);

    // Enviar el archivo en fragmentos
    for chunk in buffer.chunks(250 * 1024) {
        socket.write_all(chunk)?;
    }

    println!("Archivo enviado correctamente");

    fs::remove_file(&tar_path)?;

    Ok(())
}



pub fn create_tarball(input_file: &str) -> io::Result<String> {
    let output_file = format!("tmp.tar.gz");

    let tar_file = File::create(&output_file)?;
    let enc = GzEncoder::new(tar_file, Compression::default());
    let mut builder = Builder::new(enc);

    let file_path = Path::new(input_file);
    let file_name = get_filename(input_file);

    let mut file = File::open(file_path)?;
    builder.append_file(file_name, &mut file)?;

    Ok(output_file)
}

fn get_filename(file_path: &str) -> String {
    PathBuf::from(file_path)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_owned()
}