use std::io;

fn main() -> io::Result<()> {
    let mut string = String::from("This is a string");

    pass_by_ref(&mut string);
    let s = pass_by_val(string);

    // println!("{}", string);
    println!("{}", s);

    Ok(())
}

pub fn pass_by_ref(x: &mut String) {
    x.push_str(" which has been appended");
}

pub fn pass_by_val(x: String) -> String {
    println!("X is the owner of String string");
    x
}