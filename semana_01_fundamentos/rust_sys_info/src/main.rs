use std::io;

use std::io;

fn main() {
    let cpu_info: (u32, f64, bool) = (4, 3.2, true);
    let nucleos: u8 = cpu_info.0; // Intento de asignación u32 a u8 sin 'as'

    println!("Cores: {}", nucleos);
}

/*fn main() {
    println!("=== RUST SYS INFO ===");
    println!("Ingresa tu nombre:");

    let mut nombre = String::new();
    io::stdin()
        .read_line(&mut nombre)
        .expect("Error al leer la entrada");

    let nombre = nombre.trim();

    let uso_cpu: [u8; 4] = [23, 45, 12, 67];
    println!("{}", uso_cpu[10]);

    //let consultas = 1;

    //let consultas = consultas.to_string();

    //println!("Usuario: {}", nombre);
    //println!("Consultas realizadas: {}", consultas);
}
    */
/*
fn main() {
    let edad = 30;
    edad = 31;
    println!("{}", edad);
}
*/
