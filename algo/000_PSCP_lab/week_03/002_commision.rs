/*

Write a program to calculate commission for the input value of sales amount.
Commission is calculated as per the following rules:

i) Commission is NIL for sales amount Rs. 5000.
ii) Commission is 2% for sales when sales amount is >Rs. 5000 and <= Rs. 10000.
iii) Commission is 5% for sales amount >Rs. 10000.

 */

fn main() {
    let mut amount_str: String = "".to_string();
    std::io::stdin().read_line(&mut amount_str).unwrap();
    amount_str = amount_str.trim().to_string();
    match amount_str.parse::<u32>() {
        Ok(amount) => {
            if amount < 5000 {
                println!("No commission amount for you!");
            } else if amount >= 5000 && amount <= 10000 {
                println!("Commission amount: {}", (2 * amount) / 100);
            } else {
                println!("Commission amount: {}", (5 * amount) / 100);
            }
        }
        Err(e) => {
            eprintln!("Please enter valid amount\n{:?}", e);
        }
    }
}
