// CAPÍTULO 20 - RETO 1: Servidor HTTP de un solo hilo con manejo de errores
// Proyecto: rust_http_server

/*
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
*/

// CAPÍTULO 20 - RETO 2: Enrutado avanzado y validación de peticiones
// Proyecto: rust_http_server

use std::{
    fmt,
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
};

// 1. Estructura de la petición con datos prestados (&'a str)
#[derive(Debug)]
struct Peticion<'a> {
    metodo: &'a str,
    ruta: &'a str,
}

// 2. Enum de errores específicos de parseo
#[derive(Debug, PartialEq)]
enum ErrorPeticion {
    Vacia,
    MalFormada,
}

impl fmt::Display for ErrorPeticion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorPeticion::Vacia => write!(f, "Petición vacía"),
            ErrorPeticion::MalFormada => write!(f, "Petición mal formada"),
        }
    }
}

// 3. Función pura de parseo que devuelve Result
fn parsear(texto: &str) -> Result<Peticion<'_>, ErrorPeticion> {
    if texto.trim().is_empty() {
        return Err(ErrorPeticion::Vacia);
    }

    let linea = texto.lines().next().ok_or(ErrorPeticion::MalFormada)?;
    let mut partes = linea.split_whitespace();

    let metodo = partes.next().ok_or(ErrorPeticion::MalFormada)?;
    let ruta = partes.next().ok_or(ErrorPeticion::MalFormada)?;
    let _version = partes.next().ok_or(ErrorPeticion::MalFormada)?;

    Ok(Peticion { metodo, ruta })
}

// 4. Función pura que genera la respuesta HTTP completa según la petición
fn respuesta(texto: &str) -> String {
    let (estado, cuerpo) = match parsear(texto) {
        Ok(p) => match (p.metodo, p.ruta) {
            ("GET", "/") => ("HTTP/1.1 200 OK", "hola".to_string()),
            ("GET", _) => (
                "HTTP/1.1 404 Not Found",
                "<h1>404 No encontrado</h1>".to_string(),
            ),
            _ => (
                "HTTP/1.1 405 Method Not Allowed",
                "<h1>405 Método no permitido</h1>".to_string(),
            ),
        },
        Err(ErrorPeticion::Vacia) => (
            "HTTP/1.1 400 Bad Request",
            "<h1>400 Petición vacía</h1>".to_string(),
        ),
        Err(ErrorPeticion::MalFormada) => (
            "HTTP/1.1 400 Bad Request",
            "<h1>400 Petición mal formada</h1>".to_string(),
        ),
    };

    format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        estado,
        cuerpo.len(),
        cuerpo
    )
}

// 5. Manejo de red (E/S) limpio y sin unwrap ni expect
fn manejar(mut flujo: TcpStream) -> io::Result<()> {
    let mut buf = [0u8; 1024];
    let n = flujo.read(&mut buf)?;

    if n == 0 {
        return Ok(()); // Cliente cerró la conexión sin enviar nada
    }

    let peticion_texto = String::from_utf8_lossy(&buf[..n]);
    println!("--- Petición recibida ---\n{}", peticion_texto);

    let resp = respuesta(&peticion_texto);
    flujo.write_all(resp.as_bytes())?;
    Ok(())
}

fn main() -> io::Result<()> {
    let oyente = TcpListener::bind("127.0.0.1:7878")?;
    println!("Servidor HTTP con enrutamiento escuchando en http://127.0.0.1:7878");

    // Recorremos las conexiones entrantes de forma controlada
    for conexion in oyente.incoming().take(10) {
        match conexion {
            Ok(flujo) => {
                if let Err(e) = manejar(flujo) {
                    eprintln!("Error al procesar la conexión: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Error en el socket entrante: {}", e);
            }
        }
    }

    Ok(())
}
