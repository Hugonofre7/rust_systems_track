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
