// CAPÍTULO 20 - RETO 1: Servidor HTTP de un solo hilo con manejo de errores
// Proyecto: rust_http_server

use std::{
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
};

fn manejar(mut flujo: TcpStream) -> io::Result<()> {
    let mut buf = [0u8; 1024];
    let n = flujo.read(&mut buf)?;

    if n == 0 {
        return Ok(());
    }

    println!("{}", String::from_utf8_lossy(&buf[..n]));

    let cuerpo = "hola";
    let resp = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        cuerpo.len(),
        //10, Prueba de longitud de contenido incorrecta
        cuerpo
    );

    flujo.write_all(resp.as_bytes())?;
    Ok(())
}

fn main() -> io::Result<()> {
    let oyente = TcpListener::bind("127.0.0.1:7878")?;
    println!("Servidor HTTP escuchando en http://127.0.0.1:7878");

    for conexion in oyente.incoming().take(3) {
        match conexion {
            Ok(flujo) => {
                if let Err(e) = manejar(flujo) {
                    eprintln!("Error al procesar la petición: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Error en la conexión entrante: {}", e);
            }
        }
    }

    Ok(())
}
