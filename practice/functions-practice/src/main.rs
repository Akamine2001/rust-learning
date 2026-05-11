fn greet() {
    println!("こんにちは");
}

fn square(x: i32) -> i32 {
    x * x
}

fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn find_first_over_100_mul3() -> i32 {
    let mut n = 0;
    loop {
        n += 1;
        if n >= 100 && n % 3 == 0 {
            return n;
        }
    }
}

fn print_events(limit: i32) {
    for n in 0..=limit {
        if is_even(n) {
            println!("{}", n);
        }
    }
}

fn main() {
    greet();
    println!("{}", square(4));
    println!("{}", is_even(7));
    println!("{}", find_first_over_100_mul3());
    print_events(10);
}
