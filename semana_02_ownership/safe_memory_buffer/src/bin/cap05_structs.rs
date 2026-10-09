// CAPÍTULO 5 - RETO 1/4: Structs y Struct Update Syntax (Move Semantics)
// Proyecto: safe_memory_buffer
/*
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

    //let p = Paquete {
    //    id: 1,
    //  nombre: String::from("Paquete_Inmutable"),
    //  activo: true,
    //};

    //p.id = 5; // <--- Intento de mutar un campo de variable declarada sin 'mut'
}
*/
/*
// CAPÍTULO 5 - RETO 2/4: Tuple Structs, Unit-like Structs y Traits (Debug / Display)
// Proyecto: safe_memory_buffer

#[derive(Debug)]
struct Puerto(u16);

struct Latencia(u16);

// Unit-like Struct (Zero-Sized Type)
struct Marca;

fn conectar(p: Puerto) {
    println!("Conectando al puerto: {:?}", p);
}

fn main() {
    println!("-- Reto 2/4: Medición de Memoria & ZST --");

    let p = Puerto(8080);
    conectar(p);

    // Medición de footprint en memoria
    println!("Puerto ocupa {} bytes", std::mem::size_of::<Puerto>());
    println!("u16 ocupa {} bytes", std::mem::size_of::<u16>());
    println!("Marca (ZST) ocupa {} bytes", std::mem::size_of::<Marca>());

    // ERRORES VERIFICADOS (Comentados):
    // conectar(Latencia(8080));     // Falla E0308: Mismatched Types
    // println!("{}", Puerto(8080));  // Falla E0277: Missing Display trait
}
*/

// CAPÍTULO 5 - RETO 3/4: Métodos vs Funciones Asociadas y Ownership
// Proyecto: safe_memory_buffer

struct Contador {
    valor: u32,
}

impl Contador {
    fn nuevo() -> Self {
        Self { valor: 0 }
    }

    fn valor(&self) -> u32 {
        self.valor
    }

    fn incrementar(&mut self) {
        self.valor += 1;
    }

    fn consumir(self) -> u32 {
        self.valor
    }
}

fn main() {
    println!("-- Reto 3/4: Métodos y Ownership --");

    // PASO A: Instancia mutable, incremento y lectura con &self
    let mut c = Contador::nuevo();
    c.incrementar();
    println!("Valor de c: {}", c.valor());

    // PASO B: Consumir (Move) y posterior intento de lectura
    //let n = c.consumir();
    //println!("{} {}", n, c.valor());

    // PASO C: Variable inmutable llamando a un método que requiere &mut self
    //let c = Contador::nuevo();
    //c.incrementar();
    //println!("Valor de c: {}", c.valor());

    // PASO D: Intentar llamar a una función asociada (sin self) como método
    //c.nuevo();
    //println!("Valor de c: {}", c.valor());
}
