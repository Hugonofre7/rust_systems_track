// CAPÍTULO 13 - RETO 1/4 (Paso A): Introducción a Closures y Capturas
// Proyecto: concurrent_port_scanner
/*
fn main() {
    let umbral = 50;
    let es_alto = |x: u32| x > umbral;
    println!("{} {}", es_alto(70), es_alto(20));

    let mut llamadas = 0;
    let mut contar = || {
        llamadas += 1;
    };
    contar();
    contar();
    println!("{}", llamadas);

    // Verificamos tamaños en memoria
    let sin_captura = |x: u32| x + 1;
    println!(
        "Tamaño de es_alto (captura entorno): {} bytes",
        std::mem::size_of_val(&es_alto)
    );
    println!(
        "Tamaño de sin_captura (ZST): {} bytes",
        std::mem::size_of_val(&sin_captura)
    );
}
*/

// CAPÍTULO 13 - RETO 1/4 (Paso B): Probar préstamo mutable activo
// CAPÍTULO 13 - RETO 1/4 (Paso C): Declarar closure sin mut
/*
fn main() {
    let mut llamadas = 0;
    //let mut contar = || {
    //llamadas += 1;
    //};
    // Declaramos sin mut, pero el closure modifica el entorno
    let contar = || {
        llamadas += 1;
    };

    contar();

    //println!("Llamadas intermedias: {}", llamadas);

    contar();
    println!("Llamadas: {}", llamadas);
    //contar();
    //println!("Total llamadas: {}", llamadas);
}
*/

// CAPÍTULO 13 - RETO 1/4 (Paso D): Closures, Fn vs FnMut en el escáner
/*
fn contar_si<F: FnMut(u32) -> bool>(puertos: &[u32], mut criterio: F) -> usize {
    let mut contador = 0;
    for &p in puertos {
        if criterio(p) {
            contador += 1;
        }
    }
    contador
}

fn main() {
    let puertos = &[22, 80, 443, 8080];
    let limite = 100;

    // 1. Prueba con closure que solo lee el entorno (Fn)
    let total_menores = contar_si(puertos, |p| p < limite);
    println!("Puertos menores a {}: {}", limite, total_menores);

    // 2. Prueba intentando pasar un closure que modifica una variable capturada
    let mut puertos_evaluados = 0;
    let total_filtrados = contar_si(puertos, |p| {
        puertos_evaluados += 1;
        p > 80
    });

    println!(
        "Puertos filtrados: {}, Evaluados: {}",
        total_filtrados, puertos_evaluados
    );
}
*/

// CAPÍTULO 13 - RETO 2/4 (Paso A): Iteradores, adaptadores y consumo
// Proyecto: concurrent_port_scanner
// CAPÍTULO 13 - RETO 2/4 (Paso B): Quitar .collect()
// CAPÍTULO 13 - RETO 2/4 (Paso C): Consumidor .sum()
/*
fn main() {
    let puertos = vec![22, 80, 443, 8080, 3306];

    //let v = puertos.iter().filter(|p| **p < 1024).map(|p| {
    //    println!("proceso {}", p);
    //    p * 2
    //});

    //println!("antes de consumir");

    //let resultado: Vec<u32> = v.collect();
    //println!("{:?}", resultado);
    let total: u32 = puertos.iter().filter(|p| **p < 1024).map(|p| *p).sum();

    println!("Suma total de puertos filtrados: {}", total);
}
*/
// CAPÍTULO 13 - RETO 2/4 (Paso D - Sub-paso 1): Generar candidatos
// Proyecto: concurrent_port_scanner

fn candidatos(inicio: u32, fin: u32, excluidos: &[u32]) -> Vec<u32> {
    (inicio..=fin).filter(|p| !excluidos.contains(p)).collect()
}

// 2. Función optimizada que solo cuenta al vuelo (sin reservar memoria)
fn contar_candidatos(inicio: u32, fin: u32, excluidos: &[u32]) -> usize {
    (inicio..=fin).filter(|p| !excluidos.contains(p)).count()
}

fn main() {
    let lista = candidatos(20, 30, &[22, 25]);
    println!("Candidatos: {:?}", lista);

    let total = contar_candidatos(20, 30, &[22, 25]);
    println!(
        "Total de candidatos (sin clonar ni reservar Vec): {}",
        total
    );
}
