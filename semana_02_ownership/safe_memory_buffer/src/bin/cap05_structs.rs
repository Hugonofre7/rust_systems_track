// CAPÍTULO 5 - RETO 1/4: Structs y Struct Update Syntax (Move Semantics)
// Proyecto: safe_memory_buffer

struct Paquete {
    id: u64,
    nombre: String,
    activo: bool,
}

fn main() {
    let a = Paquete {
        id: 101,
        nombre: String::from("Buffer_Red"),
        activo: true,
    };

    // Struct Update Syntax (mueve 'nombre' y copia 'activo')
    let b = Paquete { id: 202, ..a };

    // a.id sigue siendo accesible porque u64 implementa Copy
    println!("a.id sigue siendo válido (Copy): {}", a.id);
    println!(
        "b.id: {}, b.nombre: {}, b.activo: {}",
        b.id, b.nombre, b.activo
    );

    // println!("a.nombre fallará porque fue movido a 'b': {}", a.nombre);

    let p = Paquete {
        id: 1,
        nombre: String::from("Paquete_Inmutable"),
        activo: true,
    };

    p.id = 5; // <--- Intento de mutar un campo de variable declarada sin 'mut'
}
