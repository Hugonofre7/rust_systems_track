// CAPÍTULO 10 - RETO 1/4: Traits, Coherencia y std::error::Error
// Proyecto: safe_file_rotator
/*
use std::{fs::File, io, process::ExitCode};

#[derive(Debug)]
enum ErrorRotador {
    RutaVacia,
    Io { ruta: String, fuente: io::Error },
}

impl std::fmt::Display for ErrorRotador {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorRotador::RutaVacia => write!(f, "la ruta del archivo está vacía"),
            ErrorRotador::Io { ruta, fuente } => write!(f, "{ruta}: {fuente}"),
        }
    }
}

// Contrato de error con implementación de source()
impl std::error::Error for ErrorRotador {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ErrorRotador::RutaVacia => None,
            ErrorRotador::Io { fuente, .. } => Some(fuente), // Conecta con la causa raíz
        }
    }
}

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
/*
impl Resumen for Informe {
    fn resumir(&self) -> String {
        format!("{} líneas", self.lineas)
    }
}

impl std::fmt::Display for Vec<u32> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "vector")
    }
}

fn main() {
    let i = Informe { lineas: 42 };
    println!("{} / {}", i.titulo(), i.resumir());
}
*/
// El main ahora acepta Box<dyn std::error::Error> propagando el error con ?
fn main() -> Result<(), Box<dyn std::error::Error>> {
    rotar("no_existe.log")?;
    Ok(())
}
*/

/*
// CAPÍTULO 10 - RETO 2/4: Genéricos, Traits Bounds y Monomorfización
// Proyecto: safe_file_rotator

//use std::fmt::Display;

use std::io;

#[derive(Debug)]
enum ErrorRotador {
    RutaVacia,
    Io { ruta: String, fuente: io::Error },
}

impl std::fmt::Display for ErrorRotador {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorRotador::RutaVacia => write!(f, "la ruta del archivo está vacía"),
            ErrorRotador::Io { ruta, fuente } => write!(f, "{ruta}: {fuente}"),
        }
    }
}

impl std::error::Error for ErrorRotador {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ErrorRotador::RutaVacia => None,
            ErrorRotador::Io { fuente, .. } => Some(fuente),
        }
    }
}
/*
fn mayor<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}
// Quitamos el trait bound : PartialOrd intencionalmente
//fn mayor<T>(a: T, b: T) -> T {
//    if a > b { a } else { b }
//}

fn mostrar<T: Display>(x: T) -> String {
    format!("<{}>", x)
}

fn main() {
    //println!("{}", mayor(3, 7));
    //println!("{}", mayor(2.5, 1.5));
    //println!("{}", mostrar("hola"));
    // Intentamos pasar un i32 (3) y un f64 (2.5) al mismo T
    println!("{}", mayor(3, 2.5));
}
*/
// Función genérica exigiendo el trait bound std::error::Error
fn imprimir_cadena<E: std::error::Error>(e: &E) {
    println!("Error principal: {e}");

    // Recorremos la cadena de causas usando source() de forma iterativa
    let mut causa_actual: Option<&(dyn std::error::Error + 'static)> = e.source();
    while let Some(causa) = causa_actual {
        println!("  -> Causa raíz: {causa}");
        causa_actual = causa.source();
    }
}

fn main() {
    println!("--- Probando RutaVacia ---");
    let err_vacio = ErrorRotador::RutaVacia;
    imprimir_cadena(&err_vacio);

    println!("\n--- Probando Io con archivo inexistente ---");
    let err_io = ErrorRotador::Io {
        ruta: "no_existe.log -> no_existe.log.1".to_string(),
        fuente: io::Error::new(io::ErrorKind::NotFound, "No such file or directory"),
    };
    imprimir_cadena(&err_io);
}
*/

// CAPÍTULO 10 - RETO 3/4 (Paso A): Lifetimes y Referencias
// Proyecto: safe_file_rotator

// 1. Extraer nombre de archivo (Elisión automática de lifetimes: 1 entrada -> 1 salida)
fn nombre_archivo(ruta: &str) -> &str {
    ruta.rsplit('/').next().unwrap_or(ruta)
}

// 2. Ruta o defecto (Múltiples entradas: requiere lifetimes explícitos 'a)
fn ruta_o_defecto<'a>(ruta: &'a str, defecto: &'a str) -> &'a str {
    if ruta.is_empty() { defecto } else { ruta }
}

fn main() {
    let ruta_completa = "/var/log/safe_file_rotator/app.log";
    let archivo = nombre_archivo(ruta_completa);
    println!("Nombre del archivo extraído: {}", archivo);

    let ruta_vacia = "";
    let ruta_fallback = "default.log";
    let resultado_ruta = ruta_o_defecto(ruta_vacia, ruta_fallback);
    println!("Ruta final seleccionada: {}", resultado_ruta);
}
/*
fn mas_larga<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}
fn mas_larga(x: &str, y: &str) -> &str {
if x.len() >= y.len() { x } else { y }
}

fn main() {
    let a = String::from("rotador");
    let resultado;
    {
        let b = String::from("log");
        resultado = mas_larga(&a, &b);
    }
    println!("{}", resultado);
}
*/
