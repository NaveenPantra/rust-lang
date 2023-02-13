/*

Mr. Gupta deposited Rs.1000 in a bank. The bank gives simple interest at the rate of 15% per annum.
Write a program to determine the amount in Mr. Gupta’s account at the end of 5 years.

*/

fn main() {
    let principle_amt = 1000;
    let rate_of_interest = 15;
    let time_period_in_years = 5;
    let simple_interest: f64 = (principle_amt * time_period_in_years * rate_of_interest) as f64 / 100_f64;
    println!("Simple Interest of {rate_of_interest}% with principle amount of {principle_amt} after {time_period_in_years} years is {simple_interest}");
}
