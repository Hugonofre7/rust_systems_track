struct Campo(u8);

impl Drop for Campo {
    fn drop(&mut self) {
        println!("drop Campo {}", self.0);
    }
}

struct Buffer {
    a: Campo,
    b: Campo,
}

impl Drop for Buffer {
    fn drop(&mut self) {
        println!("drop Buffer");
    }
}

fn main() {
    let _x = Buffer {
        a: Campo(1),
        b: Campo(2),
    };
    println!("fin de main");
}
