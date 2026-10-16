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
/*
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
*/

/*
use std::collections::HashMap;

fn main() {
    let mut cuenta: HashMap<String, u32> = HashMap::new();
    let clave = String::from("escribir");
    cuenta.insert(clave, 1); // clave se MUEVE al mapa
    *cuenta.entry(String::from("escribir")).or_insert(0) += 1; // existe: suma 1
    *cuenta.entry(String::from("mover")).or_insert(0) += 1; // no existe: inserta 0, suma 1
    println!("{:?}", cuenta.get("escribir")); // get acepta &str
    println!("{:?}", cuenta.get("ayuda"));
    println!("{}", cuenta.len());
    //println!("{}", clave);

    // --- NUEVAS INSERCIONES (Paso C) ---
    cuenta.insert(String::from("uno"), 10);
    cuenta.insert(String::from("dos"), 20);
    cuenta.insert(String::from("tres"), 30);
    cuenta.insert(String::from("cuatro"), 40);
    cuenta.insert(String::from("cinco"), 50);

    println!("{:?}", cuenta);
}
*/

// CAPÍTULO 8 - RETO 4/4: HashMap y API entry
// Proyecto: resilient_cli_parser

use std::collections::HashMap;

// Función que recibe los argumentos y devuelve un HashMap con las frecuencias
fn contar_argumentos(args: &[String]) -> HashMap<String, u32> {
    let mut conteo = HashMap::new();
    for arg in args {
        *conteo.entry(arg.clone()).or_insert(0) += 1;
    }
    conteo
}

// Función para imprimir el mapa en orden alfabético estable
fn imprimir_ordenado(conteo: &HashMap<String, u32>) {
    // 1. Extraemos las claves en un vector
    let mut claves: Vec<&String> = conteo.keys().collect();

    // 2. Ordenamos el vector de referencias a claves
    claves.sort();

    // 3. Iteramos de forma ordenada para mostrar resultados estables
    println!("--- Frecuencias Ordenadas ---");
    for clave in claves {
        if let Some(freq) = conteo.get(clave) {
            println!("{}: {}", clave, freq);
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let argumentos_a_procesar = if args.len() > 1 { &args[1..] } else { &[] };

    let resultado = contar_argumentos(argumentos_a_procesar);
    //println!("{:?}", resultado);
    imprimir_ordenado(&resultado);
}
