use std::io::{self, Read};

fn main() -> io::Result<()> {
    println!("Hello, world!");
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    println!("{}", buffer);
    Ok(())
}
