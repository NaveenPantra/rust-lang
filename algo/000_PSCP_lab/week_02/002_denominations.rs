/*

A cashier has currency notes of denominations Rs.10, Rs. 50 and Rs. 100.
If the amount to be withdrawn is input in hundreds,
find the total number of notes of each denomination the cashier will have to give to
the withdrawer.

*/

use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Denominations {
    Ones = 1,
    Twos = 2,
    Fives = 5,
    Tens = 10,
    Twenties = 20,
    Fifties = 50,
    Hundreds = 100,
    TwoHundreds = 200,
    FiveHundreds = 500,
    TwoThousands = 2000,
}

const DENOMINATIONS_LIST: [Denominations; 10] = [
    Denominations::TwoThousands,
    Denominations::FiveHundreds,
    Denominations::TwoHundreds,
    Denominations::Hundreds,
    Denominations::Fifties,
    Denominations::Twenties,
    Denominations::Tens,
    Denominations::Fives,
    Denominations::Twos,
    Denominations::Ones,
];

fn main() {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    match args.pop() {
        Some(amt) => {
            let amount = amt.parse::<u32>().expect("Incorrect value for amount!");
            dbg!(get_denominations(amount));
        }
        _ => {
            eprintln!("Incorrect value for amount!");
            std::process::exit(1);
        }
    }
}

fn get_denominations(mut money: u32) -> HashMap<Denominations, u32> {
    let mut required_denominations: HashMap<Denominations, u32> = HashMap::new();
    for denomination in DENOMINATIONS_LIST {
        let quotient = money / denomination as u32;
        money -= quotient * denomination as u32;
        required_denominations.insert(denomination, quotient);
    }
    required_denominations
}
