use std::io;

fn main() {
    let mut second_last = 1;
    let mut last = 1;

    println!("_Program to obtain the nth fibonacci number_");
    println!("Input n: ");

    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let n: u32 = n.trim().parse().expect("Please type a number!");

    if n == 0 {
        println!("No, seriously?");
    } else if n == 1 || n == 2 {
        println!("1");
    } else {
        let iterations = (n + 1) / 2;
        for _number in 1..iterations {
            second_last = second_last + last;
            last = second_last + last;
        }

        println!("{}", if n % 2 == 0 { last } else { second_last });
    }
}
