/*
    Write a program to generate all the prime numbers between 1 and N,
    where N is a value supplied by the user.
 */

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let n = match args.as_slice() {
        [n] => n.parse::<u8>().expect("Please enter a 8 bit positive integer."),
        _ => panic!("Please enter a positive 8 bit integer. eg: 129"),
    };
    let mut primes: Vec<u8> = vec![];
    let mut counter: u8 = 2;
    while counter <= n {
        if is_prime(counter) {
            primes.push(counter);
        }
        counter += 1;
    }
    println!("{:?}", primes);
}

// naive
fn is_prime(n: u8) -> bool {
    if n != 2 && n % 2 == 0 {
        return false;
    }
    let sqrt = (n as f64).sqrt() as u8;
    let mut counter = 3;
    while counter <= sqrt {
        if n % counter == 0 {
            return false;
        }
        counter += 1;
    }
    true
}
