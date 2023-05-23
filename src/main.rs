use std::env::consts;

fn main() {
    println!("Ahoy there!");
    println!("You be running on an OS ({}) with an ARCH of ({})!", consts::OS, consts::ARCH);
}