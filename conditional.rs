use std::io;

fn main() -> io::Result<()> {
    println!("Conditional statements");

    let bool = true;
    let mut counter = 0;

    let arr = vec![1, 2, 3, 4, 5];
    let arr2 = vec![6, 7, 8, 9, 10];
    let mut index = 0; 

    if bool {
        println!("The boolean is true");
    } else {
        println!("The boolean is false");
    }

    loop {
        println!("The counter is: {}", counter);
        counter+=1;
        if counter == 5 {
            break; 
        }
    }

    while index < arr.len() {
        println!("{}", arr[index]);
        index+=1;
    }

    for i in arr2 {
        println!("{i}")
    }

    println!("Out of the loop");

    Ok(())
}