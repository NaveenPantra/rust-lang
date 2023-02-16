/*

 If cost price and selling price of an item is input through the keyboard,
 write program to determine whether the seller has made profit or incurred loss.
 Also determine how much profit or loss he incurred in
 percentage.

 */

fn main() {
    let mut cp_sp: [f64; 2] = [0., 0.];
    for (index, arg) in std::env::args().skip(1).enumerate() {
        if index > 1 {
            break;
        }
        match arg.parse::<f64>() {
            Ok(val) => {
                cp_sp[index] = val;
            }
            Err(err) => {
                eprintln!("Error: Invalid number {:?}", err);
                std::process::exit(1);
            }
        }
    }
    let cp: f64 = cp_sp[0];
    let sp: f64 = cp_sp[1];
    // what is abs for float without crate like <u32>.abs_diff(<u32>) ?
    let mut diff = (cp - sp);
    if diff < 0_f64 {
        diff *= -1_f64;
    }
    if cp > sp {
        println!("Loss!");
        println!("Amt: {diff}");
        println!("%: {}%", ((cp - sp) / cp) * 100_f64);
    } else if sp > cp {
        println!("Profit!");
        println!("Amt: {diff}");
        println!("%: {}%", ((sp - cp) / cp) * 100_f64);
    } else {
        println!("No profit/loss incurred");
    }
}
