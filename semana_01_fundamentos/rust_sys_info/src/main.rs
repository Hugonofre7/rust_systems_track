use std::io;

fn promedio_uso(lecturas: &[u8]) -> f64 {
    let mut suma: u16 = 0;
    for lectura in lecturas {
        suma += *lectura as u16;
    }
    suma as f64 / lecturas.len() as f64
}

fn main() {
    println!("=== RUST SYS INFO ===");
    println!("Ingresa tu nombre:");

    let mut nombre = String::new();
    io::stdin()
        .read_line(&mut nombre)
        .expect("Error al leer la entrada");

    let nombre = nombre.trim();

    // Telemetría y conversión mediante shadowing
    let consultas = 1;
    let consultas = consultas.to_string();

    // Tupla de CPU: (núcleos u32, frecuencia GHz f64, es_64bits bool)
    let cpu_info: (u32, f64, bool) = (4, 3.2, true);
    let nucleos: u8 = cpu_info.0 as u8;

    // Métricas por núcleo
    let uso_cpu: [u8; 4] = [23, 45, 12, 67];

    // Cálculo con función explícita (devuelve f64)
    let promedio = promedio_uso(&uso_cpu);

    // Expresión if-else consistente (ambas ramas devuelven &str)
    let estado = if promedio > 50.0 { "ALTO" } else { "BAJO" };

    println!("---------------------------------");
    println!("Usuario: {}", nombre);
    println!("Consultas acumuladas: {}", consultas);
    println!(
        "Arquitectura: Cores: {}, Frecuencia: {} GHz, 64-bit: {}",
        nucleos, cpu_info.1, cpu_info.2
    );
    println!(
        "Uso por núcleo: Core 0: {}%, Core 1: {}%, Core 2: {}%, Core 3: {}%",
        uso_cpu[0], uso_cpu[1], uso_cpu[2], uso_cpu[3]
    );
    println!("Promedio de carga total: {:.2}% [{}]", promedio, estado);
}
