use std::thread;
use std::time::Duration;

fn sleeper(sleep: u64) {
    thread::sleep(Duration::from_secs(sleep));
}

fn largest_prime_below(max_value: u64) -> Option<u64> {
    if max_value < 2 {
        return None;
    }

    // We'll start at the maximum value and work our way down
    for num in (2..max_value).rev() {
        let mut is_prime = true;

        // Check if the current number is prime
        for i in 2..num {
            if num % i == 0 {
                is_prime = false;
                break;
            }
        }

        if is_prime {
            return Some(num);
        }
    }

    None
}

fn main() {
    let sleep_interval: u64 = 1;
    println!("Hello, world!");
    let mut largest_prime = largest_prime_below(387654321);
    sleeper(sleep_interval);
    println!("Fibonacci final value : {:?}", largest_prime);
}
