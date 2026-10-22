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

// CAPÍTULO 9 - RETO 3/4: Tipo de error propio y conversión con From
// Proyecto: safe_file_rotator
/*
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
*/

// CAPÍTULO 9 - RETO 4/4: Manejo avanzado de errores, contexto, Drop y ExitCode
// Proyecto: safe_file_rotator

use std::{fs::File, io, process::ExitCode};

#[derive(Debug)]
enum ErrorRotador {
    RutaVacia, // <--- Faltaba agregar esta variante aquí
    Io { ruta: String, fuente: io::Error },
}

// PASO A: Nuevo Display adaptado para la variante con campos con nombre y RutaVacia
impl std::fmt::Display for ErrorRotador {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorRotador::RutaVacia => write!(f, "la ruta del archivo está vacía"),
            ErrorRotador::Io { ruta, fuente } => {
                write!(f, "{}: {}", ruta, fuente)
            }
        }
    }
}

// Paso D.1: Función abrir con validación de ruta vacía y contexto de I/O
fn abrir(ruta: &str) -> Result<File, ErrorRotador> {
    if ruta.is_empty() {
        return Err(ErrorRotador::RutaVacia);
    }
    File::open(ruta).map_err(|e| ErrorRotador::Io {
        ruta: ruta.to_string(),
        fuente: e,
    })
}

// Paso D.2: Función rotar que renombra ruta a ruta + ".1" con contexto
fn rotar(ruta: &str) -> Result<(), ErrorRotador> {
    if ruta.is_empty() {
        return Err(ErrorRotador::RutaVacia);
    }
    let destino = format!("{}.1", ruta);
    std::fs::rename(ruta, &destino).map_err(|e| ErrorRotador::Io {
        ruta: format!("{} -> {}", ruta, destino),
        fuente: e,
    })
}
struct GuardianRecurso;

impl Drop for GuardianRecurso {
    fn drop(&mut self) {
        println!("-> ¡Se ejecutó el Drop! (Liberando recursos limpiamente)");
    }
}

fn main() -> ExitCode {
    // Prueba 1: Ruta vacía (debería dar error RutaVacia -> código 1)
    // Prueba 2: Archivo inexistente (debería dar error Io -> código 2)
    let ruta_a_probar = "no_existe.log"; // Cambia a "" para probar RutaVacia

    match rotar(ruta_a_probar) {
        Ok(_) => {
            println!("¡Rotación exitosa!");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("error de rotación: {e}");
            match e {
                ErrorRotador::RutaVacia => ExitCode::from(1),
                ErrorRotador::Io { .. } => ExitCode::from(2),
            }
            // Terminación abrupta del proceso a nivel de SO
            //std::process::exit(2);
        }
    }
}
