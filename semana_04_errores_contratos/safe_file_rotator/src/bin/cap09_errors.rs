/*
// CAPÍTULO 9 - RETO 1/4: panic! frente a Result<T, E> y ErrorKind
// Proyecto: safe_file_rotator

use std::fs::File;
use std::io::ErrorKind;

fn main() {
    File::open("no_existe.log").unwrap();
    /*match File::open("no_existe.log") {
        Ok(_f) => println!("abierto"),
        Err(e) => match e.kind() {
            ErrorKind::NotFound => println!("no existe"),
            otro => println!("otro error: {:?}", otro),
        },
    }
    */
}
*/
// CAPÍTULO 9 - RETO 2/4: Propagación de errores con el operador ?
// Proyecto: safe_file_rotator

/*
use std::fs::File;
use std::io::Read;

fn leer_archivo(ruta: &str) -> Result<String, std::io::Error> {
    let mut f = File::open(ruta)?;
    let mut contenido = String::new();
    f.read_to_string(&mut contenido)?;
    Ok(contenido)
}

/*
fn main() {
    match leer_archivo("no_existe.log") {
        Ok(c) => println!("{} bytes", c.len()),
        Err(e) => eprintln!("error: {}", e),
    }

fn main() {
    let c = leer_archivo("no_existe.log")?;
    println!("{}", c.len());
*/

fn main() -> Result<(), std::io::Error> {
    let c = leer_archivo("no_existe.log")?;
    println!("{}", c.len());
    Ok(())
}
*/

use std::{fs::File, io};

//#[derive(Debug)]
enum ErrorRotador {
    Io(io::Error),
    RutaVacia,
}

impl From<io::Error> for ErrorRotador {
    fn from(e: io::Error) -> Self {
        ErrorRotador::Io(e)
    }
}

// PASO D1: Implementación parcial de Display (faltará cubrir Io)
/*
impl std::fmt::Display for ErrorRotador {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorRotador::RutaVacia => write!(f, "La ruta del archivo está vacía"),
        }
    }
}
*/
// PASO D2: Implementación completa de Display (match exhaustivo sin comodín _)
impl std::fmt::Display for ErrorRotador {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorRotador::RutaVacia => write!(f, "La ruta del archivo está vacía"),
            ErrorRotador::Io(err) => write!(f, "Error de I/O en el sistema: {}", err),
        }
    }
}

fn abrir(ruta: &str) -> Result<File, ErrorRotador> {
    if ruta.is_empty() {
        return Err(ErrorRotador::RutaVacia);
    }
    Ok(File::open(ruta)?)
}

fn main() {
    //println!("{:?}", abrir(""));
    //println!("{:?}", abrir("no_existe.log"));
    if let Err(e) = abrir("") {
        println!("{}", e);
    }

    if let Err(e) = abrir("no_existe.log") {
        println!("{}", e);
    }
}
