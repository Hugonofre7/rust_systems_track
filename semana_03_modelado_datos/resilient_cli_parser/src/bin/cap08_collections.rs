/*
fn main() {
    println!(
        "Tamaño de Vec<u32> en bytes: {}",
        std::mem::size_of::<Vec<u32>>()
    );

    // Cambiamos Vec::new() por Vec::with_capacity(9)
    let mut v: Vec<u32> = Vec::with_capacity(9);
    println!(
        "Capacidad inicial: {}, Longitud inicial: {}",
        v.capacity(),
        v.len()
    );

    for i in 1..=9 {
        v.push(i);
        println!(
            "Push {i} -> Longitud: {}, Capacidad: {}",
            v.len(),
            v.capacity()
        );
    }
    //let primero = &v[0];
    //v.push(99);
    //println!("{}", primero);
}
*/

fn main() {
    let mut v = vec![10, 20, 30];
    match v.get(5) {
        Some(x) => println!("{}", x),
        None => println!("no existe el índice 5"),
    }
    for x in &mut v {
        *x += 1;
    }
    for x in v {
        println!("{}", x);
    }
    println!("{}", v[1]);
    //println!("{}", v[5]);
}
