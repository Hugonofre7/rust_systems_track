// CAPÍTULO 8 - RETO 1/4: Vectores, Crecimiento en Heap y Reglas de Préstamo
// Proyecto: resilient_cli_parser

/*
fn main() {
    println!("Tamaño de Vec<u32> en bytes: {}", std::mem::size_of::<Vec<u32>>());

    // Usando Vec::with_capacity(9) para evitar reasignaciones en el heap
    let mut v: Vec<u32> = Vec::with_capacity(9);
    println!("Capacidad inicial: {}, Longitud inicial: {}", v.capacity(), v.len());

    for i in 1..=9 {
        v.push(i);
        println!("Push {i} -> Longitud: {}, Capacidad: {}", v.len(), v.capacity());
    }
}
*/
/*
// CAPÍTULO 8 - RETO 2/4: Acceso Seguro con get(), Pánicos y Semántica de Movimiento
// Proyecto: resilient_cli_parser

fn main() {
    let mut v = vec![10, 20, 30];

    // Acceso seguro con get() en lugar de indexación directa que causa pánico
    match v.get(5) {
        Some(x) => println!("{}", x),
        None => println!("no existe el índice 5"),
    }

    // Modificación mediante referencia mutable
    for x in &mut v {
        *x += 1;
    }

    // Iteración inmutable para mostrar los elementos actualizados
    for x in &v {
        println!("{}", x);
    }

    println!("{}", v[1]);

    // Demostración del pánico en runtime por índice fuera de límites (Paso B)
    // println!("{}", v[5]);

    /*
    // Demostración del error E0382 por mover el vector al iterar sin referencia (Paso C)
    for x in v {
        println!("{}", x);
    }
    println!("{}", v.len());
    */
}
*/

// CAPÍTULO 8 - RETO 3/4: Strings, UTF-8, Bytes vs Caracteres y Slicing Seguro
// Proyecto: resilient_cli_parser

fn main() {
    let s = String::from("Hola, ñandú");
    println!("{} {}", s.len(), s.chars().count());
    println!("{:?}", s.chars().nth(6));
    let corte = &s[0..4];
    println!("{}", corte);
    let t = String::from("ñ");
    println!("{}", t.len());
    //println!("{}", &s[6..7]);
    println!("{:?}", s.get(6..7));
    let c = s[0];
}
