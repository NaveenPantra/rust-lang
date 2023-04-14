// Write a program to find the sum of individual digits of a positive integer.

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut number = match args.as_slice() {
        [number] => number.parse::<u128>().expect(&format!("{number} is not a 128bit positive integer.")),
        _ => panic!("Please give a positive number. eg: 345"),
    };
    let mut result = 0;
    while number != 0 {
        let rem = number % 10;
        result += rem;
        number /= 10;
    }
    println!("Sum of individual digits of {} is {result}", args[0]);
}
