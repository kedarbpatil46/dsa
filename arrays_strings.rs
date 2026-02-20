use std::io;

fn main() -> io::Result<()> {
    let string = String::from("kedar");

    let arr = vec![1, 2, 3, 4];

    let slice = &string[0..1];
    
    println!("String slice: {}", slice);
    println!("Length of string: {}", string.len());

    println!("{}", arr[0]);
    println!("{}", arr.len());

    Ok(())
}