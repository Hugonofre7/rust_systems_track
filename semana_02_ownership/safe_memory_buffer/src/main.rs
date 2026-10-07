// ============================================================================
// CAPÍTULO 4: Ownership, Move Semantics y Copy Trait
// Proyecto: safe_memory_buffer
// ============================================================================

fn toma_posesion_string(buffer: String) {
    println!("String recibido en función: {}", buffer);
} // Aquí 'buffer' sale de scope y se invoca 'drop', liberando la memoria en el Heap.

fn toma_posesion_i32(buffer: i32) {
    println!("i32 recibido en función: {}", buffer);
}

fn main() {
    // ------------------------------------------------------------------------
    // PASO A: Demostración de Move Semantics (Heap Allocation)
    // ------------------------------------------------------------------------
    let dato_string = String::from("telemetría crítica");

    // Transfiere la propiedad (Ownership) a la función.
    toma_posesion_string(dato_string);

    // [DEMOSTRACIÓN DE ERROR COMPILACIÓN E0382]:
    // La siguiente línea falla porque 'dato_string' fue movido.
    // Rust previene Double Free invalidando la variable en el Stack Frame local.
    //
    // println!("Error E0382: {}", dato_string);

    // ------------------------------------------------------------------------
    // PASO B: Demostración de Copy Trait (Stack-Only Data)
    // ------------------------------------------------------------------------
    let dato_i32 = 42;

    // Los tipos primitivos en el Stack implementan 'Copy'. Se realiza bitwise copy.
    toma_posesion_i32(dato_i32);

    // 'dato_i32' sigue siendo válido en el Stack de main():
    println!("i32 retenido en main() tras llamada: {}", dato_i32);
}
