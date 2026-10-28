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
/*
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
*/

// CAPÍTULO 13 - RETO 3: Variantes de Iteradores (.iter, .iter_mut, .into_iter)
// CAPÍTULO 13 - PASO B: into_iter() y el error de propiedad (E0382)
// Proyecto: concurrent_port_scanner

/*
fn main() {
    let mut puertos = vec![80, 443, 22];
    /*
    for p in puertos.iter() {
    print!("{} ", p);
    }
    println!();

    for p in puertos.iter_mut() {
    *p += 1;
    }
    */

    let propios: Vec<u32> = puertos.into_iter().map(|p| p * 2).collect();

    println!("Puertos duplicados: {:?}", puertos);

    // Paso B: Descomentamos el uso de puertos tras ser consumido
    println!(
        "Intentando usar 'puertos' después de into_iter: {:?}",
        puertos
    );

    //let propios: Vec<u32> = puertos.into_iter().map(|p| p * 2).collect();
    // println!("{:?}", propios);
}
*/

// CAPÍTULO 13 - PASO C: Mutación concurrente y el Borrow Checker (E0502)
// Proyecto: concurrent_port_scanner
/*
fn main() {
    let mut puertos = vec![80, 443, 22];

    // .iter() mantiene un préstamo inmutable activo de todo el vector
    for p in puertos.iter() {
        // Paso C: Intentamos mutar el vector mientras está prestado inmutablemente
        puertos.push(*p);
    }
}
*/

// CAPÍTULO 13 - RETO 3: Variantes de Iteradores y Seguridad de Memoria
// Proyecto: concurrent_port_scanner
/*
// 1. Modifica los puertos en sitio usando iter_mut() sobre un slice mutable
fn aplicar_offset(puertos: &mut [u32], offset: u32) {
    for p in puertos.iter_mut() {
        *p += offset;
    }
}

// 2. Devuelve un String con los puertos formateados usando iter() sobre un slice inmutable
fn resumen(puertos: &[u32]) -> String {
    let partes: Vec<String> = puertos.iter().map(|p| p.to_string()).collect();
    partes.join(", ")
}

fn main() {
    // -----------------------------------------------------------------
    // Demostración conceptual de las 3 variantes de iteradores
    // -----------------------------------------------------------------
    let mut puertos = vec![80, 443, 22];

    // A) .iter(): Préstamo inmutable (&u32)
    print!("Puertos iniciales: ");
    for p in puertos.iter() {
        print!("{} ", p);
    }
    println!();

    // B) .iter_mut(): Préstamo mutable (&mut u32) - Modificación en sitio
    for p in puertos.iter_mut() {
        *p += 1;
    }
    println!("Puertos modificados (iter_mut): {:?}", puertos);

    // C) .into_iter(): Consumo por valor (u32) - Mueve la propiedad del Vec
    let propios: Vec<u32> = puertos.into_iter().map(|p| p * 2).collect();
    println!("Puertos duplicados (into_iter): {:?}", propios);

    // -----------------------------------------------------------------
    // Aplicación arquitectónica para el concurrent_port_scanner
    // -----------------------------------------------------------------
    println!("\n--- Aplicación en el escáner ---");
    let mut puertos_scanner = vec![22, 80];
    let offset = 1000;

    println!("Antes del offset: [{}]", resumen(&puertos_scanner));

    aplicar_offset(&mut puertos_scanner, offset);

    println!("Después del offset (1000): [{}]", resumen(&puertos_scanner));
}
*/

// CAPÍTULO 13 - RETO 4 (Paso A): Particionado con .chunks()
// CAPÍTULO 13 - RETO 4 (Paso B): Comportamiento de .chunks(0)
// CAPÍTULO 13 - RETO 4 (Paso C): Mutación prohibida durante chunks()
// Proyecto: concurrent_port_scanner
/*
fn main() {
    let puertos: Vec<u32> = (20..=30).collect();

    for (_i, grupo) in puertos.chunks(4).enumerate() {
        // Paso B: Intentamos usar chunks con tamaño 0
        //for (i, grupo) in puertos.chunks(0).enumerate() {
        let suma: u32 = grupo.iter().sum();
        //println!("grupo {}: {:?} suma={}", i, grupo, suma);
        println!("Suma: {}", suma);
        // Paso C: Intentamos mutar el vector mientras chunks() mantiene un préstamo inmutable
        puertos.push(1);
    }

    //let total = puertos.chunks(4).count();
    //println!("{} grupos", total);
}
*/

// =====================================================================
// CAPÍTULO 13 - RETO 4 (Paso D): Repartición de trabajo y Concurrencia
// Proyecto: concurrent_port_scanner
// =====================================================================

// Reparte un slice de puertos entre N hilos usando chunks y redondeo hacia arriba

// CAPÍTULO 13 - RETO 4 (Paso D): Repartición de trabajo y Concurrencia
// Proyecto: concurrent_port_scanner

fn repartir(puertos: &[u32], hilos: usize) -> Option<Vec<Vec<u32>>> {
    if hilos == 0 {
        return None;
    }

    // Cálculo seguro del tamaño de trozo redondeando hacia arriba
    let chunk_size = (puertos.len() + hilos - 1) / hilos;
    let chunk_size = if chunk_size == 0 { 1 } else { chunk_size };

    let grupos: Vec<Vec<u32>> = puertos
        .chunks(chunk_size)
        .map(|chunk| chunk.to_vec())
        .collect();

    Some(grupos)
}

fn main() {
    // 1. Generar candidatos de 1..=30 excluyendo [22, 25]
    let candidatos: Vec<u32> = (1..=30).filter(|p| *p != 22 && *p != 25).collect();

    // 2. Filtrar y contar usando un closure que captura limite = 24
    let limite = 24;
    let total_menores = candidatos.iter().filter(|&&p| p < limite).count();
    println!("Candidatos menores a {}: {}\n", limite, total_menores);

    // 3. Repartir los candidatos entre 4 hilos
    let hilos = 4;
    println!(
        "Repartiendo {} puertos entre {} hilos:",
        candidatos.len(),
        hilos
    );

    match repartir(&candidatos, hilos) {
        Some(grupos) => {
            for (i, grupo) in grupos.iter().enumerate() {
                println!("  -> Grupo/Hilo {}: {:?}", i, grupo);
            }
        }
        None => println!("Error: El número de hilos no puede ser cero."),
    }
}
