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
