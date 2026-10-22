// CAPÍTULO 10 - RETO 1/4: Traits, Coherencia y std::error::Error
// Proyecto: safe_file_rotator

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
