use std::io;

fn main() -> io::Result<()> {
    let mut input1 = String::new();
    let mut input2 = String::new();
    println!("Enter number 1: ");
    io::stdin().read_line(&mut input1).unwrap();

    println!("Enter number 2: ");
    io::stdin().read_line(&mut input2).unwrap();

    let a = match input1.trim().parse() {
        Ok(val) => val,
        Err(err) => panic!("{}", err),
    };

    let b = match input2.trim().parse() {
        Ok(val) => val,
        Err(err) => panic!("{}", err),
    };

    find_gcd(a, b);

    Ok(())
}

fn find_gcd(x: usize, y: usize) {
    let mut gcd = 1;

    if x < y {
        for i in 1..(x/2) {
            if x%i == 0 && y%i == 0 {
                gcd = i;
            } 
        }
    } else {
        for i in 1..(y/2) {
            if x%i == 0 && y%i == 0 {
                gcd = i;
            } 
        }
    }

    println!("The GCD is: {}", gcd);
}
