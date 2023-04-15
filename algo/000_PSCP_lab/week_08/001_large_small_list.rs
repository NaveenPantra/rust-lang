/*
    Write a program to find the largest and smallest number in a given list of integers.
*/

fn main() {
    let list: Vec<i32> = vec![0, -34, 34, 54, 23, -09, 234, 0, 0, 023, -234, -3, -1235, 9094];
    let mut smallest: i32 = std::i32::MAX;
    let mut largest: i32 = std::i32::MIN;
    for num in list {
        if num < smallest {
            smallest = num
        }
        if num > largest {
            largest = num
        }
    }
    println!("Largest: {largest}, Smallest: {smallest}");
}
