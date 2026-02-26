use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();
    println!("Enter a number: ");

    io::stdin().read_line(&mut input).unwrap();

    let x = match input.trim().parse() {
        Ok(val) => val,
        Err(err) => panic!("{}", err)
    };
    
    count_digits(x);
    Ok(())
}

fn count_digits(x: usize) {
    let mut counter = 0;
    let mut val = x;
    while val > 0{
        val = val/10;
        counter = counter + 1;
    }

    println!("The number of digits of number {} is {}", x, counter);
}