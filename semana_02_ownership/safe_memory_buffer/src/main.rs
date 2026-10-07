/*
// CAPÍTULO 4: Ownership, Move Semantics y Copy Trait
// Proyecto: safe_memory_buffer
//

fn toma_posesion_string(buffer: String) {
    println!("String recibido en función: {}", buffer);
} // Aquí 'buffer' sale de scope y se invoca 'drop', liberando la memoria en el Heap.

fn toma_posesion_i32(buffer: i32) {
    println!("i32 recibido en función: {}", buffer);
}

fn main() {
    //
    // PASO A: Demostración de Move Semantics (Heap Allocation)
    //
    let dato_string = String::from("telemetría crítica");

    // Transfiere la propiedad (Ownership) a la función.
    toma_posesion_string(dato_string);

    // [DEMOSTRACIÓN DE ERROR COMPILACIÓN E0382]:
    // La siguiente línea falla porque 'dato_string' fue movido.
    // Rust previene Double Free invalidando la variable en el Stack Frame local.
    //
    // println!("Error E0382: {}", dato_string);

    //
    // PASO B: Demostración de Copy Trait (Stack-Only Data)
    //
    let dato_i32 = 42;

    // Los tipos primitivos en el Stack implementan 'Copy'. Se realiza bitwise copy.
    toma_posesion_i32(dato_i32);

    // 'dato_i32' sigue siendo válido en el Stack de main():
    println!("i32 retenido en main() tras llamada: {}", dato_i32);
}
*/

// ============================================================================
// CAPÍTULO 4 - RETO 2/3: Referencias, Préstamo (& / &mut) y Borrow Checker
// Proyecto: safe_memory_buffer
// ============================================================================

fn main() {
    let mut log = String::from("evento_red");

    //
    // DEMOSTRACIÓN DE ERROR E0502 (Comentado para compilación):
    // Violación de Aliasing + Mutabilidad
    //
    // let r1 = &log;
    // let r2 = &mut log; // Falla: préstamo mutable mientras existe &log
    // println!("{} {}", r1, r2);

    //
    // PASO A & B: Referencias inmutables múltiples e inmutabilidad seguida de mutabilidad
    //
    let r1 = &log;
    let r2 = &log;

    println!("Lecturas inmutables simultáneas -> r1: {}, r2: {}", r1, r2);
    // <- Fin del lifetime activo de r1 y r2 gracias a NLL

    let r3 = &mut log;
    r3.push_str(" :: CRÍTICO");

    println!("Escritura mutable exclusiva -> r3: {}", r3);
}
