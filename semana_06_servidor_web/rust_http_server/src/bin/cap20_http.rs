// CAPÍTULO 20 - RETO 1: Servidor HTTP de un solo hilo con manejo de errores
// Proyecto: rust_http_server

/*
use std::{
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
};

fn manejar(mut flujo: TcpStream) -> io::Result<()> {
    let mut buf = [0u8; 1024];
    let n = flujo.read(&mut buf)?;

    if n == 0 {
        return Ok(());
    }

    println!("{}", String::from_utf8_lossy(&buf[..n]));

    let cuerpo = "hola";
    let resp = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        cuerpo.len(),
        //10, Prueba de longitud de contenido incorrecta
        cuerpo
    );

    flujo.write_all(resp.as_bytes())?;
    Ok(())
}

fn main() -> io::Result<()> {
    let oyente = TcpListener::bind("127.0.0.1:7878")?;
    println!("Servidor HTTP escuchando en http://127.0.0.1:7878");

    for conexion in oyente.incoming().take(3) {
        match conexion {
            Ok(flujo) => {
                if let Err(e) = manejar(flujo) {
                    eprintln!("Error al procesar la petición: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Error en la conexión entrante: {}", e);
            }
        }
    }

    Ok(())
}
*/

// CAPÍTULO 20 - RETO 2: Enrutado avanzado y validación de peticiones
// Proyecto: rust_http_server
/*
use std::{
    fmt,
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
};

// 1. Estructura de la petición con datos prestados (&'a str)
#[derive(Debug)]
struct Peticion<'a> {
    metodo: &'a str,
    ruta: &'a str,
}

// 2. Enum de errores específicos de parseo
#[derive(Debug, PartialEq)]
enum ErrorPeticion {
    Vacia,
    MalFormada,
}

impl fmt::Display for ErrorPeticion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorPeticion::Vacia => write!(f, "Petición vacía"),
            ErrorPeticion::MalFormada => write!(f, "Petición mal formada"),
        }
    }
}

// 3. Función pura de parseo que devuelve Result
fn parsear(texto: &str) -> Result<Peticion<'_>, ErrorPeticion> {
    if texto.trim().is_empty() {
        return Err(ErrorPeticion::Vacia);
    }

    let linea = texto.lines().next().ok_or(ErrorPeticion::MalFormada)?;
    let mut partes = linea.split_whitespace();

    let metodo = partes.next().ok_or(ErrorPeticion::MalFormada)?;
    let ruta = partes.next().ok_or(ErrorPeticion::MalFormada)?;
    let _version = partes.next().ok_or(ErrorPeticion::MalFormada)?;

    Ok(Peticion { metodo, ruta })
}

// 4. Función pura que genera la respuesta HTTP completa según la petición
fn respuesta(texto: &str) -> String {
    let (estado, cuerpo) = match parsear(texto) {
        Ok(p) => match (p.metodo, p.ruta) {
            ("GET", "/") => ("HTTP/1.1 200 OK", "hola".to_string()),
            ("GET", _) => (
                "HTTP/1.1 404 Not Found",
                "<h1>404 No encontrado</h1>".to_string(),
            ),
            _ => (
                "HTTP/1.1 405 Method Not Allowed",
                "<h1>405 Método no permitido</h1>".to_string(),
            ),
        },
        Err(ErrorPeticion::Vacia) => (
            "HTTP/1.1 400 Bad Request",
            "<h1>400 Petición vacía</h1>".to_string(),
        ),
        Err(ErrorPeticion::MalFormada) => (
            "HTTP/1.1 400 Bad Request",
            "<h1>400 Petición mal formada</h1>".to_string(),
        ),
    };

    format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        estado,
        cuerpo.len(),
        cuerpo
    )
}

// 5. Manejo de red (E/S) limpio y sin unwrap ni expect
fn manejar(mut flujo: TcpStream) -> io::Result<()> {
    let mut buf = [0u8; 1024];
    let n = flujo.read(&mut buf)?;

    if n == 0 {
        return Ok(()); // Cliente cerró la conexión sin enviar nada
    }

    let peticion_texto = String::from_utf8_lossy(&buf[..n]);
    println!("--- Petición recibida ---\n{}", peticion_texto);

    let resp = respuesta(&peticion_texto);
    flujo.write_all(resp.as_bytes())?;
    Ok(())
}

fn main() -> io::Result<()> {
    let oyente = TcpListener::bind("127.0.0.1:7878")?;
    println!("Servidor HTTP con enrutamiento escuchando en http://127.0.0.1:7878");

    // Recorremos las conexiones entrantes de forma controlada
    for conexion in oyente.incoming().take(10) {
        match conexion {
            Ok(flujo) => {
                if let Err(e) = manejar(flujo) {
                    eprintln!("Error al procesar la conexión: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Error en el socket entrante: {}", e);
            }
        }
    }

    Ok(())
}
*/

// CAPÍTULO 20 - RETO 3: Demostración de bloqueo por una petición lenta
// Proyecto: rust_http_server
/*
use std::{
    fmt,
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::{Duration, Instant},
};

#[derive(Debug)]
struct Peticion<'a> {
    metodo: &'a str,
    ruta: &'a str,
}

#[derive(Debug, PartialEq)]
enum ErrorPeticion {
    Vacia,
    MalFormada,
}

impl fmt::Display for ErrorPeticion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorPeticion::Vacia => write!(f, "Petición vacía"),
            ErrorPeticion::MalFormada => write!(f, "Petición mal formada"),
        }
    }
}

fn parsear(texto: &str) -> Result<Peticion<'_>, ErrorPeticion> {
    if texto.trim().is_empty() {
        return Err(ErrorPeticion::Vacia);
    }

    let linea = texto.lines().next().ok_or(ErrorPeticion::MalFormada)?;
    let mut partes = linea.split_whitespace();

    let metodo = partes.next().ok_or(ErrorPeticion::MalFormada)?;
    let ruta = partes.next().ok_or(ErrorPeticion::MalFormada)?;
    let _version = partes.next().ok_or(ErrorPeticion::MalFormada)?;

    Ok(Peticion { metodo, ruta })
}

fn respuesta(texto: &str) -> String {
    let (estado, cuerpo) = match parsear(texto) {
        Ok(p) => match (p.metodo, p.ruta) {
            ("GET", "/") => ("HTTP/1.1 200 OK", "hola".to_string()),
            ("GET", "/lento") => {
                // Simulamos trabajo lento que bloquea el hilo actual
                thread::sleep(Duration::from_secs(3));
                (
                    "HTTP/1.1 200 OK",
                    "procesamiento lento terminado".to_string(),
                )
            }
            ("GET", _) => (
                "HTTP/1.1 404 Not Found",
                "<h1>404 No encontrado</h1>".to_string(),
            ),
            _ => (
                "HTTP/1.1 405 Method Not Allowed",
                "<h1>405 Método no permitido</h1>".to_string(),
            ),
        },
        Err(ErrorPeticion::Vacia) => (
            "HTTP/1.1 400 Bad Request",
            "<h1>400 Petición vacía</h1>".to_string(),
        ),
        Err(ErrorPeticion::MalFormada) => (
            "HTTP/1.1 400 Bad Request",
            "<h1>400 Petición mal formada</h1>".to_string(),
        ),
    };

    format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        estado,
        cuerpo.len(),
        cuerpo
    )
}

fn manejar(mut flujo: TcpStream) -> io::Result<()> {
    flujo.set_read_timeout(Some(Duration::from_secs(2)))?;
    let inicio = Instant::now();
    let mut buf = [0u8; 1024];

    // Opcional para el punto D.1 (lo usaremos enseguida):
    // flujo.set_read_timeout(Some(Duration::from_secs(2)))?;

    let n = flujo.read(&mut buf)?;

    if n == 0 {
        return Ok(());
    }

    let peticion_texto = String::from_utf8_lossy(&buf[..n]);
    let resp = respuesta(&peticion_texto);

    flujo.write_all(resp.as_bytes())?;
    println!("Conexión atendida en {:?}", inicio.elapsed());
    Ok(())
}

fn main() -> io::Result<()> {
    let oyente = TcpListener::bind("127.0.0.1:7878")?;
    println!("Servidor enlazado en http://127.0.0.1:7878");

    for conexion in oyente.incoming() {
        match conexion {
            Ok(flujo) => {
                if let Err(e) = manejar(flujo) {
                    eprintln!("Error al procesar la conexión: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Error en el socket entrante: {}", e);
            }
        }
    }

    Ok(())
}
*/

// CAPÍTULO 20 - RETO 4: Construcción del Thread Pool
// Proyecto: rust_http_server
/*
use std::{
    fmt,
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex, mpsc},
    thread,
    time::{Duration, Instant},
};

type Trabajo = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    emisor: mpsc::Sender<Trabajo>,
}

#[derive(Debug)]
pub enum ErrorPool {
    ZeroThreads,
    EnvioFallido,
}

impl ThreadPool {
    pub fn nuevo(n: usize) -> Result<ThreadPool, ErrorPool> {
        if n == 0 {
            return Err(ErrorPool::ZeroThreads);
        }

        let (emisor, receptor) = mpsc::channel::<Trabajo>();
        let receptor = Arc::new(Mutex::new(receptor));

        for _ in 0..n {
            let rx = Arc::clone(&receptor);
            thread::spawn(move || {
                loop {
                    let trabajo = {
                        let rx = rx.lock().unwrap();
                        rx.recv().unwrap()
                    }; // El cerrojo se suelta aquí al salir del bloque
                    trabajo();
                }
            });
            //thread::spawn(move || {
            //while let Ok(trabajo) = rx.lock().unwrap().recv() {
            //trabajo();
            //loop {
            //let trabajo = rx.lock().unwrap().recv().unwrap();
            //trabajo();
            //}
            //});
        }

        Ok(ThreadPool { emisor })
    }

    pub fn ejecutar<F>(&self, f: F) -> Result<(), ErrorPool>
    where
        F: FnOnce() + Send + 'static,
    {
        self.emisor
            .send(Box::new(f))
            .map_err(|_| ErrorPool::EnvioFallido)
    }
}

#[derive(Debug)]
struct Peticion<'a> {
    metodo: &'a str,
    ruta: &'a str,
}

#[derive(Debug, PartialEq)]
enum ErrorPeticion {
    Vacia,
    MalFormada,
}

impl fmt::Display for ErrorPeticion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorPeticion::Vacia => write!(f, "Petición vacía"),
            ErrorPeticion::MalFormada => write!(f, "Petición mal formada"),
        }
    }
}

fn parsear(texto: &str) -> Result<Peticion<'_>, ErrorPeticion> {
    if texto.trim().is_empty() {
        return Err(ErrorPeticion::Vacia);
    }

    let linea = texto.lines().next().ok_or(ErrorPeticion::MalFormada)?;
    let mut partes = linea.split_whitespace();

    let metodo = partes.next().ok_or(ErrorPeticion::MalFormada)?;
    let ruta = partes.next().ok_or(ErrorPeticion::MalFormada)?;
    let _version = partes.next().ok_or(ErrorPeticion::MalFormada)?;

    Ok(Peticion { metodo, ruta })
}

/*
fn respuesta(texto: &str) -> String {
    let (estado, cuerpo) = match parsear(texto) {
        Ok(p) => match (p.metodo, p.ruta) {
            ("GET", "/") => ("HTTP/1.1 200 OK", "hola".to_string()),
            ("GET", "/lento") => {
                thread::sleep(Duration::from_secs(3));
                (
                    "HTTP/1.1 200 OK",
                    "procesamiento lento terminado".to_string(),
                )
            }
            ("GET", _) => (
                "HTTP/1.1 404 Not Found",
                "<h1>404 No encontrado</h1>".to_string(),
            ),
            _ => (
                "HTTP/1.1 405 Method Not Allowed",
                "<h1>405 Método no permitido</h1>".to_string(),
            ),
        },
        Err(ErrorPeticion::Vacia) => (
            "HTTP/1.1 400 Bad Request",
            "<h1>400 Petición vacía</h1>".to_string(),
        ),
        Err(ErrorPeticion::MalFormada) => (
            "HTTP/1.1 400 Bad Request",
            "<h1>400 Petición mal formada</h1>".to_string(),
        ),
    };

    format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        estado,
        cuerpo.len(),
        cuerpo
    )
}
*/

fn respuesta(texto: &str) -> String {
    let (estado, cuerpo) = match parsear(texto) {
        Ok(p) => match (p.metodo, p.ruta) {
            ("GET", "/") => ("HTTP/1.1 200 OK", "hola".to_string()),
            ("GET", "/lento") => {
                thread::sleep(Duration::from_secs(3));
                (
                    "HTTP/1.1 200 OK",
                    "procesamiento lento terminado".to_string(),
                )
            }
            ("GET", "/panico") => {
                panic!("¡Pánico intencional en el hilo obrero!");
            }
            _ => ("HTTP/1.1 404 NOT FOUND", "404.html".to_string()),
        },
        Err(_) => ("HTTP/1.1 400 BAD REQUEST", "400.html".to_string()),
    };

    format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        estado,
        cuerpo.len(),
        cuerpo
    )
}

fn manejar(mut flujo: TcpStream) -> io::Result<()> {
    flujo.set_read_timeout(Some(Duration::from_secs(2)))?;
    let inicio = Instant::now();
    let mut buf = [0u8; 1024];

    let n = flujo.read(&mut buf)?;
    if n == 0 {
        return Ok(());
    }

    let peticion_texto = String::from_utf8_lossy(&buf[..n]);
    let resp = respuesta(&peticion_texto);

    flujo.write_all(resp.as_bytes())?;
    println!("Conexión atendida en {:?}", inicio.elapsed());
    Ok(())
}

fn main() -> io::Result<()> {
    let oyente = TcpListener::bind("127.0.0.1:7878")?;
    println!("Servidor enlazado en http://127.0.0.1:7878");

    // Creamos el thread pool con 4 obreros
    let pool = ThreadPool::nuevo(4).unwrap();
    //let pool = ThreadPool::nuevo(2).unwrap();

    //for conexion in oyente.incoming() {
    for conexion in oyente.incoming().take(2) {
        match conexion {
            Ok(flujo) => {
                // Entregamos el trabajo al pool en lugar de procesarlo directamente en main
                pool.ejecutar(move || {
                    if let Err(e) = manejar(flujo) {
                        eprintln!("Error al procesar la conexión: {}", e);
                    }
                })
                .unwrap(); // Si el envío falla, controlamos el error
            }
            Err(e) => {
                eprintln!("Error en el socket entrante: {}", e);
            }
        }
    }
    println!("Cerrando el emisor del pool...");
    drop(pool); // Esto destruye el ThreadPool y por tanto su emisor

    // Esperamos medio segundo para ver qué imprimen los obreros
    thread::sleep(Duration::from_millis(500));
    Ok(())
}
*/

// CAPÍTULO 20 - RETO 5: Cierre ordenado y elegante con Drop
// Proyecto: rust_http_server

use std::{
    fmt, fs,
    io::{self, Read, Write, prelude::*},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex, mpsc},
    thread,
    time::{Duration, Instant},
};

type Trabajo = Box<dyn FnOnce() + Send + 'static>;

// Estructura Obrero para gestionar cada hilo individualmente con Option
struct Obrero {
    id: usize,
    hilo: Option<thread::JoinHandle<()>>,
}

impl Obrero {
    fn nuevo(id: usize, receptor: Arc<Mutex<mpsc::Receiver<Trabajo>>>) -> Obrero {
        let hilo = thread::spawn(move || {
            loop {
                let mensaje = {
                    let rx = receptor.lock().unwrap();
                    rx.recv()
                };

                match mensaje {
                    Ok(trabajo) => {
                        println!("Obrero {id} recibió un trabajo; ejecutando.");
                        trabajo();
                    }
                    Err(_) => {
                        println!("Obrero {id} desconectado; terminando.");
                        break;
                    }
                }
            }
        });

        Obrero {
            id,
            hilo: Some(hilo),
        }
    }
}

pub struct ThreadPool {
    obreros: Vec<Obrero>,
    emisor: Option<mpsc::Sender<Trabajo>>,
    //emisor: mpsc::Sender<Trabajo>, // Sin Option
}

impl ThreadPool {
    pub fn nuevo(tamano: usize) -> Result<ThreadPool, &'static str> {
        assert!(tamano > 0);

        let (emisor, receptor) = mpsc::channel();
        let receptor = Arc::new(Mutex::new(receptor));

        let mut obreros = Vec::with_capacity(tamano);
        for id in 0..tamano {
            obreros.push(Obrero::nuevo(id, Arc::clone(&receptor)));
        }

        Ok(ThreadPool {
            obreros,
            emisor: Some(emisor),
        })
    }
    pub fn ejecutar<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let trabajo = Box::new(f);
        self.emisor.as_ref().unwrap().send(trabajo).unwrap();
    }
}

// Implementación del Drop unificada para asegurar un cierre limpio y ordenado
impl Drop for ThreadPool {
    fn drop(&mut self) {
        // 1. Cerramos el canal tomando el emisor. (Comenta esta línea si quieres probar el Paso B de deadlock)
        drop(self.emisor.take());

        println!("Cerrando el emisor del pool. Esperando a los obreros...");

        // 2. Hacemos join de cada hilo de forma segura usando Option::take
        for obrero in &mut self.obreros {
            println!("Apagando obrero {}", obrero.id);
            if let Some(hilo) = obrero.hilo.take() {
                let _ = hilo.join();
            }
        }
    }
}

// Lógica de enrutamiento y respuestas
fn respuesta(texto: &str) -> String {
    // Aquí mantienes tu lógica de parseo y rutas (/lento, /panico, etc.)
    let estado = "HTTP/1.1 200 OK";
    let cuerpo = "hola";
    format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        estado,
        cuerpo.len(),
        cuerpo
    )
}

fn manejar(mut flujo: TcpStream) -> io::Result<()> {
    let mut buffer = [0; 1024];
    flujo.read(&mut buffer)?;

    let respuesta_http = respuesta(&String::from_utf8_lossy(&buffer[..]));
    flujo.write_all(respuesta_http.as_bytes())?;
    flujo.flush()?;
    Ok(())
}

fn main() -> io::Result<()> {
    let oyente = TcpListener::bind("127.0.0.1:7878")?;
    println!("Servidor enlazado en http://127.0.0.1:7878");

    let pool = ThreadPool::nuevo(2).unwrap();

    for flujo in oyente.incoming().take(2) {
        let flujo = flujo?;
        pool.ejecutar(move || {
            if let Err(e) = manejar(flujo) {
                eprintln!("Error al procesar la conexión: {}", e);
            }
        });
    }

    println!("Fin de main, destruyendo pool...");
    Ok(())
}
