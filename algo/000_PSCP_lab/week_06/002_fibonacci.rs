/*
    A Fibonacci sequence is defined as follows:
    The first and second terms in the sequence are 0 and 1.
    Subsequent terms are found by adding the preceding two terms in the sequence.
    Write a program to generate the first N terms of the sequence.
 */

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let n = match args.as_slice() {
        [n] => n.parse::<u8>().expect("Please enter a 8bit positive integer"),
        _ => panic!("Please enter a positive integer. eg: 12"),
    };
    let mut first = -1;
    let mut second = 1;
    let mut counter: u8 = 0;
    let mut fibo = String::new();
    while counter < n {
        let next = first + second;
        first = second;
        second = next;
        fibo += &(next.to_string() + " ");
        counter += 1;
    }
    println!("{fibo}");
}
