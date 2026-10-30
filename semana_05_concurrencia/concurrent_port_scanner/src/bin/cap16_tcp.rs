// CAPÍTULO 16 - RETO 4 (Paso A): Conexión TCP y tiempo de espera
// CAPÍTULO 16 - RETO 4 (Paso B): Probando drop(oyente)
// Proyecto: concurrent_port_scanner
/*
use std::{
    net::{SocketAddr, TcpListener, TcpStream},
    time::Duration,
};

fn abierto(puerto: u16) -> bool {
    let dir = SocketAddr::from(([127, 0, 0, 1], puerto));
    TcpStream::connect_timeout(&dir, Duration::from_millis(200)).is_ok()
}

fn main() {
    let oyente = TcpListener::bind("127.0.0.1:0").unwrap();
    let puerto = oyente.local_addr().unwrap().port();

    println!("escuchando en {}", puerto);

    // Destruimos el oyente a propósito antes de escanear
    drop(oyente);

    println!("{} {}", abierto(puerto), abierto(1));
}
*/

// CAPÍTULO 16 - RETO 4 (Paso C): Escáner de puertos concurrente completo
// Proyecto: concurrent_port_scanner

use std::{
    net::{SocketAddr, TcpListener, TcpStream},
    sync::mpsc,
    thread,
    time::Duration,
};

// Función para verificar si un puerto está abierto usando TCP
fn abierto(puerto: u16) -> bool {
    let dir = SocketAddr::from(([127, 0, 0, 1], puerto));
    TcpStream::connect_timeout(&dir, Duration::from_millis(200)).is_ok()
}

// Función que escanea grupos de puertos en paralelo usando hilos y canales
fn escanear(grupos: Vec<Vec<u16>>) -> Vec<u16> {
    let (tx, rx) = mpsc::channel();

    for grupo in grupos {
        let tx_clon = tx.clone();
        thread::spawn(move || {
            for puerto in grupo {
                if abierto(puerto) {
                    // Si el canal receptor se cierra abruptamente, ignoramos el error
                    // en lugar de usar unwrap o expect, asegurando robustez.
                    if tx_clon.send(puerto).is_err() {
                        break;
                    }
                }
            }
        });
    }

    // Soltamos el emisor original para que el iterador del receptor sepa cuándo terminar
    drop(tx);

    let mut puertos_abiertos = Vec::new();
    for puerto in rx {
        puertos_abiertos.push(puerto);
    }

    // Ordenamos la lista antes de devolverla
    puertos_abiertos.sort();
    puertos_abiertos
}

fn main() {
    // 1. Creamos un oyente en un puerto aleatorio elegido por el SO
    let oyente = TcpListener::bind("127.0.0.1:0").unwrap();
    let puerto_dinamico = oyente.local_addr().unwrap().port();

    println!("Puerto dinámico abierto por el SO: {}", puerto_dinamico);

    // 2. Armamos la lista de candidatos: rango 20000..=20023 más nuestro puerto dinámico
    let mut candidatos: Vec<u16> = (20000..=20023).collect();
    candidatos.push(puerto_dinamico);

    // 3. Dividimos los candidatos en 4 grupos usando .chunks()
    let tamano_chunk = (candidatos.len() + 3) / 4;
    let grupos: Vec<Vec<u16>> = candidatos
        .chunks(tamano_chunk)
        .map(|chunk| chunk.to_vec())
        .collect();

    println!(
        "Escaneando {} puertos divididos en {} grupos...",
        candidatos.len(),
        grupos.len()
    );

    // 4. Ejecutamos nuestro escáner concurrente
    let resultados = escanear(grupos);

    println!("Puertos abiertos encontrados: {:?}", resultados);
}
