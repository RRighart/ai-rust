use std::time::{Instant};

fn fibonacci(n: u64) -> u64 {
    match n {
        1 | 0 => n,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn main() {
    let start = Instant::now();
    println!("fibonacci(40) = {}", fibonacci(40));
    let duration = start.elapsed();
    println!("Time elapsed in fibonacci() is: {:?}", duration);
}

// fibonacci(40) = 102334155
// Time elapsed in fibonacci() is: 899.509242ms