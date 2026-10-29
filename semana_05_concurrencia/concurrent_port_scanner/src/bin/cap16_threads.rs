// CAPÍTULO 16 - RETO 1 (Paso A): Hilos con thread::spawn, move y join
// CAPÍTULO 16 - RETO 1 (Paso B.1): Error al omitir 'move'
// CAPÍTULO 16 - RETO 1 (Paso B.2): Error de valor movido (E0382)
// Proyecto: concurrent_port_scanner
/*
use std::thread;

fn main() {
    let puertos = vec![22, 80, 443];

    // Lanzamos un hilo secundario y le movemos la propiedad de `puertos`
    //let h = thread::spawn(move || {
    //let suma: u32 = puertos.iter().sum();
    //suma // El valor de retorno viaja dentro del JoinHandle
    //});

    // Paso B.1: Quitamos 'move' del closure
    let h = thread::spawn(|| {
        panic!("fallo");
    });
    //let suma: u32 = puertos.iter().sum();
    //suma
    //});

    //println!("main sigue trabajando");

    // Paso B.2: Intentamos usar `puertos` en main después de moverlo al hilo
    //println!("Intentando usar puertos en main: {:?}", puertos);
    println!("Main esperando al hilo que va a fallar...");

    // Paso C: .join().unwrap() propagará el pánico del hilo secundario al hilo principal
    let _resultado = h.join().unwrap();

    println!("Esto nunca se imprimirá");

    // Esperamos a que el hilo termine y recogemos su resultado
    //let resultado = h.join().unwrap();
    //println!("suma = {}", resultado);
}
*/

// CAPÍTULO 16 - RETO 1 (Paso D): Múltiples hilos y JoinHandles
// Proyecto: concurrent_port_scanner

use std::thread;

// 1. Función que lanza un hilo para sumar un lote de puertos y devuelve el JoinHandle
fn suma_en_hilo(puertos: Vec<u32>) -> thread::JoinHandle<u32> {
    thread::spawn(move || puertos.iter().sum())
}

fn main() {
    // 2. Creamos tres grupos de puertos (simulando el particionado del Cap. 13)
    let grupos = vec![vec![22, 80], vec![443, 8080], vec![3306]];

    let mut handles = vec![];

    // Lanzamos un hilo por cada grupo usando into_iter() para transferir la propiedad
    for grupo in grupos.into_iter() {
        let handle = suma_en_hilo(grupo);
        handles.push(handle);
    }

    // 3. Recogemos los resultados de todos los hilos con join()
    let mut total_general = 0;
    for (i, h) in handles.into_iter().enumerate() {
        let resultado_hilo = h.join().unwrap();
        println!("Suma del hilo {}: {}", i, resultado_hilo);
        total_general += resultado_hilo;
    }

    println!("Suma total de todos los puertos: {}", total_general);
}
