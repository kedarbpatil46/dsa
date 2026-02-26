use std::io;

fn main() -> io::Result<()> {
    println!("Enter the number here: ");
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let x = match input.trim().parse() {
        Ok(val) => val,
        Err(err) => panic!("{}", err)
    };

    reverse_num(x);
    Ok(())
}

fn reverse_num(x: usize) {
    let mut val = x;
    let mut ans = String::new();

    while val > 0 {
        let digit = val%10;
        val = val/10;
        ans.push_str(&(digit.to_string()));
    }

    println!("The reverse of {} is {}.", x, ans);
}