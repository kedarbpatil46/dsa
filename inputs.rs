use std::io;

fn main() -> io::Result<()> {
    println!("Enter something: ");
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;

    print!("You entered: {}", input);

    Ok(())
}