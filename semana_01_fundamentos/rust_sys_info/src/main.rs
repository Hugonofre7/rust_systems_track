use std::io;

fn main() {
    println!("=== RUST SYS INFO ===");
    println!("Ingresa tu nombre:");

    let mut nombre = String::new();
    io::stdin()
        .read_line(&mut nombre)
        .expect("Error al leer la entrada");

    println!("Hola, {}!", nombre.trim());
}
