// CAPÍTULO 6 - RETO 1/4: Enums, Tamaños de Memoria y Null Pointer Optimization
// Proyecto: resilient_cli_parser

/*
use std::mem::size_of;

#[allow(dead_code)]
enum Comando {
    Ayuda,
    Ejecutar(String),
}

#[allow(dead_code)]
enum ComandoExtendido {
    Salir,
    Mover,
    Escribir,
    Ejecutar(String),
}

fn main() {
    println!("-- Reto 1/4: Análisis de Tamaños, Nichos y Cascada --");

    // Pruebas de referencias y tipos primitivos con nichos
    println!("Tamaño de &u8: {} bytes", size_of::<&u8>());
    println!("Tamaño de Option<&u8>: {} bytes", size_of::<Option<&u8>>());

    println!("Tamaño de u8: {} bytes", size_of::<u8>());
    println!("Tamaño de Option<u8>: {} bytes", size_of::<Option<u8>>());

    // Análisis de enums con String (24 bytes) y optimización de nichos
    println!("Tamaño de String: {} bytes", size_of::<String>());
    println!("Tamaño de Comando (1 unit + 1 String): {} bytes", size_of::<Comando>());
    println!("Tamaño de ComandoExtendido (3 unit + 1 String): {} bytes", size_of::<ComandoExtendido>());

    // Demostración de nichos en cascada
    println!("Option<Option<String>>: {} bytes", size_of::<Option<Option<String>>>());
}
*/

// CAPÍTULO 6 - RETO 2/4: Option<T>, Match Ergonomics y Pánicos en Runtime
// Proyecto: resilient_cli_parser

fn primer_arg(args: &[String]) -> Option<String> {
    if args.len() > 1 {
        Some(args[1].clone())
    } else {
        None
    }
}

fn main() {
    println!("-- Reto 2/4: Manejo seguro de Option vs Pánicos --");

    let args: Vec<String> = std::env::args().collect();

    // PATRÓN SEGURO: Uso de match para manejar variantes Some y None
    match primer_arg(&args) {
        Some(arg) => println!("arg: {}", arg),
        None => println!("sin argumentos"),
    }

    // ERRORES VERIFICADOS (Comentados):
    // 1. Operación aritmética directa con Option (Falla E0369):
    // let x: Option<i32> = Some(5);
    // let y = x + 1;

    // 2. Pánico en runtime por forzar unwrap sobre un valor None:
    // let a = primer_arg(&args).unwrap();
    // println!("Primer argumento: {}", a);
}
