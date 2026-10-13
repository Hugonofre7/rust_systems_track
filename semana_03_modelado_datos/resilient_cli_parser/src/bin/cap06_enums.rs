/*
use std::mem::size_of;

#[allow(dead_code)]
enum ComandoExtendido {
    Salir,
    Mover,
    Escribir,
    Ejecutar(String),
}

fn main() {
    println!("-- Reto 1/4 (Cap. 6): Análisis de Tamaños y Nichos --");

    println!("Tamaño de &u8: {} bytes", size_of::<&u8>());
    println!("Tamaño de Option<&u8>: {} bytes", size_of::<Option<&u8>>());

    println!("Tamaño de u8: {} bytes", size_of::<u8>());
    println!("Tamaño de Option<u8>: {} bytes", size_of::<Option<u8>>());

    println!("Tamaño de String: {} bytes", size_of::<String>());
    println!(
        "Tamaño de ComandoExtendido (3 unit + 1 String): {} bytes",
        size_of::<ComandoExtendido>()
    );
    println!(
        "Option<Option<String>>: {} bytes",
        size_of::<Option<Option<String>>>()
    );
}
*/
/*
fn main() {
    // Recogemos los argumentos de la línea de comandos y los convertimos en un Vector de Strings
    let args: Vec<String> = std::env::args().collect();

    println!("Argumentos recibidos: {:?}", args);
}
*/
/*
fn main() {
    let x: Option<i32> = Some(5);
    let y = x + 1;

    println!("y: {}", y);
}
*/

fn primer_arg(args: &[String]) -> Option<String> {
    if args.len() > 1 {
        Some(args[1].clone())
    } else {
        None
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Usamos match para manejar el option de forma segura y clara.
    match primer_arg(&args) {
        Some(arg) => println!("arg: {}", arg),
        None => println!("sin argumentos"),
    }
    // Sustituimos el match por .unwrap() para forzar el pánico si no hay argumentos
    //let a = primer_arg(&args).unwrap();

    //println!("Primer argumento: {}", a);
}
