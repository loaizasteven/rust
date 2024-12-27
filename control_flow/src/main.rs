fn main(){
    println!("\nStart Range Example");
    range();

    println!("\nStart Loop Example");
    loops();
}

fn range() {
    for number in 1..4 { // to reverse the range use (1..4).rev()
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

fn loops() {
    let mut count = 0;
    'counting_up: loop { //loop label
        println!("count = {count}");
        let mut remaining =10;

        loop{
            println!("remaining = {remaining}");
            if remaining == 9 {
                break; // breaks the inner loop
            }
            if count == 2 {
                break 'counting_up; // breaks the outer loop using label
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}