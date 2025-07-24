mod network;
mod utils;

fn main() {
    println!("{}", utils::binary::Binary::read_byte(0xFF, true));
     println!("{}", utils::binary::Binary::write_byte(0x41));
    println!("Hello, world!");
}
