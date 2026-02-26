use std::io;

fn main() -> io::Result<()> {
    println!("Enter string here: ");
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    input = input.trim().to_string();
    // let x = match input.trim().parse() {
    //     Ok(val) => val,
    //     Err(err) => panic!("{}", err)
    // };

    is_palindrome(input);

    Ok(())
}


fn is_palindrome(x: String) {
    let char_array: Vec<char> = x.chars().collect();
    let mut a = 0;
    let mut b = x.len() - 1;
    let mut is_palin = false;
    while a <= b {
        if char_array[a] != char_array[b] {
            is_palin = false;
            println!("The string is not a palindrome!");
            return;
            break;
        } else {
            a = a + 1;
            b = b - 1;
        }
    }
    is_palin = true;
    if is_palin {
        println!("The string is a Palindrome!");
    }
}
