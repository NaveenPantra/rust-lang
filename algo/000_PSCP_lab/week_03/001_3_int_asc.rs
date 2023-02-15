/*

Write a program that prints the given 3 integers in descending order using if - else.

*/

fn main() {
    let a: i32 = 10;
    let b: i32 = 0;
    let c: i32 = 100;

    let descending_order = if a > b {
        if b >= c {
            (a, b, c)
        } else if a < c {
            (c, a, b)
        } else {
            (a, c, b)
        }
    } else if b > c {
        if c > a {
            (b, c, a)
        } else {
            (b, a, c)
        }
    } else {
        (c, b, a)
    };

    println!("{descending_order:?}");
}
