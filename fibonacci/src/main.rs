use std::io;

fn fib(n: i32) -> i32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}

fn main() {
    let mut n = String::new();

    println!("Enter a number: ");
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let n: i32 = n.trim().parse().expect("Please type a number");
    let response = fib(n);
    println!("Fib Sequence Results for '{n}' is {response}");
}
