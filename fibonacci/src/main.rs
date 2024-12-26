use std::io;
struct Solution;

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
    let method = 2;

    println!("Enter a number: ");
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let n: i32 = n.trim().parse().expect("Please type a number");
    let response = if method == 1 {
        fib(n)
    } else {
        println!("Running Method 2: Solution::fib(n)");
        Solution::fib(n)
    };
    println!("Fib Sequence Results for '{n}' is {response}");
}

impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n == 0 {
            return 0; 
        }
        if n ==1 {
            return 1;
        }

        //set initial variables
        let mut a = 0;
        let mut b = 1;

        //loop through the sequence, inclusive of n 
        for _ in 2..=n {
            let temp = a + b;
            a=b;
            b=temp;
        }
        return b
    }
}