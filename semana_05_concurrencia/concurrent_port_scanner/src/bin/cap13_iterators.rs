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
