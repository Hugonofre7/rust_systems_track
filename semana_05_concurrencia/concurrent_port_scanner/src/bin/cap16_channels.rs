// CAPÍTULO 16 - RETO 2 (Paso A): Canales mpsc básicos
// CAPÍTULO 16 - RETO 2 (Paso B): Bloqueo por falta de drop(tx)
// CAPÍTULO 16 - RETO 2 (Prueba Paso C): Error de valor movido (E0382)
// Proyecto: concurrent_port_scanner

/*
use std::{sync::mpsc, thread};

fn lanzar(grupos: Vec<Vec<u32>>) -> mpsc::Receiver<(usize, u32)> {
    let (tx, rx) = mpsc::channel();

    for (i, grupo) in grupos.into_iter().enumerate() {
        let tx_clon = tx.clone();

        thread::spawn(move || {
            let suma: u32 = grupo.iter().sum();

            let v = String::from("puerto_critico");

            tx_clon.send((i, suma)).unwrap();

            println!("Usando v: {}", v);
        });
    }

    drop(tx);
    rx
}
fn main() {
    //let (tx, rx) = mpsc::channel();
    let grupos = vec![vec![22, 80], vec![443, 8080]];
    let rx = lanzar(grupos);

    for (indice, suma) in rx {
        println!("Grupo {}: {}", indice, suma);
    }

    //for id in 0..3 {
    //let tx = tx.clone();
    //thread::spawn(move || {
    //tx.send(id * 10).unwrap();
    //});
    //}

    //drop(tx);

    // El bucle termina automáticamente cuando todos los emisores clonados se cierran
    //for recibido in rx {
    //println!("recibido {}", recibido);
    //}
}
*/

// CAPÍTULO 16 - RETO 2 (Paso D): Integración en el escáner con lanzar y canales
// Proyecto: concurrent_port_scanner

use std::{sync::mpsc, thread};

// 1. Función que lanza hilos por cada grupo y devuelve un Receptor (Receiver)
fn lanzar(grupos: Vec<Vec<u32>>) -> mpsc::Receiver<(usize, u32)> {
    let (tx, rx) = mpsc::channel();

    for (i, grupo) in grupos.into_iter().enumerate() {
        let tx_clon = tx.clone(); // Clonamos el emisor para cada hilo

        thread::spawn(move || {
            let suma: u32 = grupo.iter().sum();
            // Enviamos una tupla con el índice del grupo y su resultado
            tx_clon.send((i, suma)).unwrap();
        });
    }

    // ¡Importante! Soltamos el emisor original para que el receptor sepa
    // que no se enviarán más mensajes y el bucle for pueda terminar.
    drop(tx);

    rx
}

fn main() {
    // 2. Definimos nuestros grupos de puertos
    let grupos = vec![vec![22, 80], vec![443, 8080], vec![3306]];

    let total_grupos = grupos.len();
    println!(
        "Lanzando procesamiento para {} grupos de puertos...",
        total_grupos
    );

    // 3. Obtenemos el receptor del canal
    let rx = lanzar(grupos);

    // 4. Procesamos los resultados conforme van llegando (sin importar el orden)
    let mut total_general = 0;
    for (indice, suma) in rx {
        println!(
            "  -> Resultado recibido del grupo {}: suma = {}",
            indice, suma
        );
        total_general += suma;
    }

    println!("\nSuma total procesada por canales: {}", total_general);
}
