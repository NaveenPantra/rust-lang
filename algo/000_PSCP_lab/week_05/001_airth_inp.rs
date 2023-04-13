/*

Write a program, which takes two integer operands and one operator
as input from the user, performs the operation and then prints the result.
(Consider the operators +,-,*, /, %. Use switch statement)

 */

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (operand1, operator, operand2): (i32, &str, i32) = match args.as_slice() {
        [operand1, operator, operand2] => (
            operand1.parse::<i32>().expect("Operand1 must be an 32bit integer"),
            operator,
            operand2.parse::<i32>().expect("Operand1 must be an 32bit integer"),
        ),
        _ => panic!("\nPlease give three args seperated by a space. Ex: 1 + 2\n")
    };
    // Default wrapping arithmetic
    let result: i32 = match operator {
        "+" => operand1 + operand2,
        "-" => operand1 - operand2,
        "*" => operand1 * operand2,
        "/" => operand1 / operand2,
        "%" => operand1 % operand2,
        _ => panic!("Please provide a valid arithmetic operator ex: +, -, *, /, %"),
    };
    println!("Result of {operand1} {operator} {operand2} = {result}");
}
