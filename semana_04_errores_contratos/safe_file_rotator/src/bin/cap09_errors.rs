use std::fs::File;
use std::io::ErrorKind;

fn main() {
    File::open("no_existe.log").unwrap();
    //match File::open("no_existe.log") {
    //Ok(_f) => println!("abierto"),
    //Err(e) => match e.kind() {
    //ErrorKind::NotFound => println!("no existe"),
    //otro => println!("otro error: {:?}", otro),
    //},
}
