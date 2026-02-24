use std::io;

fn main() -> io::Result<()> {
    println!("Enter the value: ");
    let mut val = String::new();

    io::stdin().read_line(&mut val)?;

    let cycle = match val.trim().parse() {
        Ok(val) => val,
        Err(err) => panic!("{}", err),
    };

    pattern1(cycle);
    pattern2(cycle);
    pattern3(cycle);
    pattern4(cycle);
    pattern5(cycle);
    pattern6(cycle);
    pattern7(cycle);
    pattern8(cycle);
    pattern9(cycle);
    pattern10(cycle);
    pattern11(cycle);
    pattern12(cycle);

    Ok(())
}

fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

fn pattern1(x: usize) {
    let mut ans = String::new();
    for _ in 0..x {
        ans.push_str("* ");
    }

    for _ in 0..x {
        println!("{}", ans);
    }
}

fn pattern2(x: usize) {
    let mut ans = String::new();
    for _ in 0..x {
        ans.push_str("*");
        println!("{}", ans);
    }
}

fn pattern3(x: usize) {
    let mut ans = String::new();
    let mut counter = 0;
    for _ in 0..x {
        counter = counter + 1;
        ans.push_str(&(counter.to_string()));
        println!("{}", ans);
    }
}

fn pattern4(x: usize) {
    let mut counter = 0;
    for _ in 0..x {
        let mut ans = String::new();
        counter = counter + 1;
        for _ in 0..counter {
            ans.push_str(&(counter.to_string()));
        }
        println!("{}", ans);
    }
}

fn pattern5(x: usize) {
    let mut counter = x;

    for _ in 0..counter {
        let mut ans = String::new();
        for _ in 0..counter {
            ans.push_str("*");
        }
        println!("{}", ans);
        counter = counter - 1;
    }
}

fn pattern6(x: usize) {
    let mut counter = x;

    for _ in 0..counter {
        let mut ans = String::new();
        for mut j in 0..counter {
            j = j + 1;
            ans.push_str(&(j.to_string()));
        }
        println!("{}", ans);
        counter = counter - 1;
    }
}

fn pattern7(x: usize) {
    for i in 0..x {
        for _ in 0..(x - i - 1) {
            print!(" ");
        }

        for _ in 0..(2 * i + 1) {
            print!("*");
        }
        println!();
    }
}

fn pattern8(x: usize) {
    let mut counter = x;
    for _ in 0..x {
        for _ in 0..(x - counter) {
            print!(" ");
        }
        for _ in 0..(2 * counter - 1) {
            print!("*");
        }
        counter = counter - 1;
        println!();
    }
}

fn pattern9(x: usize) {
    pattern7(x);
    pattern8(x);
}

fn pattern10(x: usize) {
    pattern2(x);
    pattern5(x - 1);
}

fn pattern11(x: usize) {
    for i in 1..(x + 1) {
        let mut ans = String::new();
        for j in 0..i {
            if i % 2 == 0 {
                if j % 2 == 0 {
                    ans.push_str("0");
                } else {
                    ans.push_str("1");
                }
            } else {
                if j % 2 == 0 {
                    ans.push_str("1");
                } else {
                    ans.push_str("0");
                }
            }
        }
        println!("{}", ans);
    }
}

fn pattern12(x: usize) {
    for i in 1..(x+1) {
        let mut ans = String::new();
        for j in 1..(x+1) {
            if j <= i {
                ans.push_str(&(j.to_string()));
            } else {
                ans.push_str(" ");
            }
        }
        let s = reverse_string(ans.as_str());
        println!("{}{}", ans, s);
    }
}

fn pattern13() {
    
}