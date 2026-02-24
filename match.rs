use std::io;

fn main() -> io::Result<()> {
    let num = 5;

    match num {
        1 => println!("The number is 1"),
        2 => println!("The number is 2"),
        3 => println!("The number is 3"),
        4 => println!("The number is 4"),
        5 => println!("The number is 5"),
        _ => println!("The number is something else!")
    };

    Ok(())
}