/*

Let a and b are two integer variables whose values are 10 and 13
respectively. Write a program to evaluate the following arithmetic expressions.

i) a + b ii) a – b iii) a * b iv) a / b v) a % b

 */

fn main() {
    let a: i16 = 10;
    let b: i16 = 13;
    let (
        addition,
        subtraction,
        multiplication,
        division,
        modulo_division,
    ) = perform_simple_arithmetic(a, b);
    println!(r"
    Addition            {0} + {1} = {addition}
    Subtraction         {0} - {1} = {subtraction}
    Multiplication      {0} * {1} = {multiplication}
    Division            {0} / {1} = {division}
    Modulo Division     {0} % {1} = {modulo_division}
    ", a, b);
}

fn perform_simple_arithmetic(a: i16, b: i16) -> (i16, i16, i16, i16, i16) {
    (a.saturating_add(b), a.saturating_sub(b), a.saturating_mul(b), a.saturating_div(b), a % b)
}
