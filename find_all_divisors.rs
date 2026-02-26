use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();
    println!("Enter a number: ");

    io::stdin().read_line(&mut input).unwrap();

    let x = match input.trim().parse() {
        Ok(val) => val,
        Err(err) => panic!("{}", err)
    };

    find_divisors(x);

    Ok(())
}

fn find_divisors(x: usize) {
    for i in 1..x {
        if x%i == 0 {
            print!("{} ", i);
        }
    }
    println!("{}", x);
}