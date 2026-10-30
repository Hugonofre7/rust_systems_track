// CAPÍTULO 16 - RETO 3 (Paso A): Estado compartido con Arc y Mutex
// CAPÍTULO 16 - RETO 3 (Prueba Paso B): Rc en lugar de Arc
// Proyecto: concurrent_port_scanner
/*
use std::rc::Rc;
use std::sync::Mutex;
use std::thread;

fn main() {
    //let contador = Arc::new(Mutex::new(0u32));
    let contador = Rc::new(Mutex::new(0u32));
    let mut hilos = Vec::new();

    for _ in 0..4 {
        //let c = Arc::clone(&contador);
        let c = Rc::clone(&contador); // <-- Rc::clone
        hilos.push(thread::spawn(move || {
            let mut guardia = c.lock().unwrap();
            *guardia += 1;
        }));
    }

    for h in hilos {
        h.join().unwrap();
    }

    println!("total = {}", *contador.lock().unwrap());
}
*/

// CAPÍTULO 16 - RETO 3 (Paso C): Sleep dentro del cerrojo (Mutex contention)
// CAPÍTULO 16 - RETO 3 (Paso C.2): Sleep fuera del cerrojo (Concurrencia real)
// Proyecto: concurrent_port_scanner

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let puertos_abiertos = Arc::new(Mutex::new(Vec::new()));
    let mut hilos = vec![];

    let inicio = Instant::now();

    for i in 0..3 {
        let puertos = Arc::clone(&puertos_abiertos);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(100));

            let puerto_encontrado = 80 + i * 10;

            // Tomamos el cerrojo ANTES de dormir
            let mut guardia = puertos.lock().unwrap();
            guardia.push(puerto_encontrado);
            // El cerrojo se libera aquí al salir del ámbito (Drop)
        });
        hilos.push(handle);
    }

    for h in hilos {
        h.join().unwrap();
    }

    let duracion = inicio.elapsed();
    let mut resultado = puertos_abiertos.lock().unwrap();
    resultado.sort();

    println!("Puertos encontrados: {:?}", *resultado);
    println!("Tiempo total con sleep Fuera del cerrojo: {:?}", duracion);
}
