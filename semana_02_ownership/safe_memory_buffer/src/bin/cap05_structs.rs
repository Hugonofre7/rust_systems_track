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
/*
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
*/

// CAPÍTULO 5 - RETO 4/4: Integración Arquitectónica y RAII (Drop Trait)
// Proyecto: safe_memory_buffer

// Newtype Pattern para el identificador de Buffer
#[derive(Debug, Clone, Copy)]
struct BufferId(u32);

// Struct principal con campos con nombre
struct BufferInspeccionado {
    id: BufferId,
    contenido: Vec<u8>,
}

impl BufferInspeccionado {
    // 1. Función asociada (Constructor)
    fn nuevo(id: u32, datos_iniciales: &[u8]) -> Self {
        Self {
            id: BufferId(id),
            contenido: datos_iniciales.to_vec(),
        }
    }

    // 2. Método con préstamo inmutable (&self): Consulta de tamaño
    fn tamano(&self) -> usize {
        self.contenido.len()
    }

    // 3. Método con préstamo mutable (&mut self): Extensión del contenido
    fn escribir(&mut self, datos: &[u8]) {
        self.contenido.extend_from_slice(datos);
    }

    // 4. Método que consume la instancia (self): Finalizar y devolver tamaño
    fn finalizar(self) -> usize {
        println!(
            "  [finalizar] Procesando y consumiendo Buffer #{:?}...",
            self.id
        );
        self.contenido.len()
        // `self` sale de ámbito AQUÍ y ejecuta `Drop`
    }
}

// Implementación del trait Drop para observar el ciclo de vida (RAII)
impl Drop for BufferInspeccionado {
    fn drop(&mut self) {
        println!("---> [DROP] Liberando Buffer #{:?} de la memoria", self.id);
    }
}

fn main() {
    println!("-- Reto 4/4: Demostración de Scopes y Drop --\n");

    // Buffer 1: Sobrevivirá hasta el final de main()
    let mut buf1 = BufferInspeccionado::nuevo(1, b"Hola ");
    buf1.escribir(b"Rust!");
    println!("Buf1 (ID 1) creado. Tamaño actual: {} bytes", buf1.tamano());

    // Scope anidado
    {
        println!("\n--- Entrando al Scope Anidado ---");
        let buf2 = BufferInspeccionado::nuevo(2, b"Datos Temporales");
        println!(
            "Buf2 (ID 2) dentro del scope. Tamaño: {} bytes",
            buf2.tamano()
        );
        println!("--- Saliendo del Scope Anidado ---");
    } // buf2 debe destruirse AQUÍ

    println!("\n--- De regreso en main() ---");

    // Buffer 3: Consumido por el método finalizar(self)
    let buf3 = BufferInspeccionado::nuevo(3, b"Buffer Final");
    println!("Buf3 (ID 3) creado. Invocando finalizar()...");
    let bytes_procesados = buf3.finalizar(); // buf3 debe destruirse DENTRO de esta llamada
    println!("Bytes devueltos por finalizar(): {}", bytes_procesados);

    println!("\n--- Finalizando main() ---");
} // buf1 debe destruirse AQUÍ
