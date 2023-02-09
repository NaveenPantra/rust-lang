use std::ops::Add;

fn main() {
    let mut arr: Vec<i32> = vec![-6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6];
    let sum: i32 = 2;
    match pair_sum(&mut arr, &sum) {
        Some((ind1, ind2)) => {
            println!("Pair found for {sum} are arr[{ind1}]: {} and  arr[{ind2}]: {}", arr[ind1], arr[ind2]);
        },
        None => {
            println!("No Pair found for {sum}");
        }
    }
}

fn pair_sum<T: Add + Copy >(arr: &mut [T], sum: &T) -> Option<(usize, usize)>
where <T as Add>::Output: PartialEq<T>, <T as Add>::Output: PartialOrd<T>
{
    let mut i = 0;
    let mut j = arr.len() - 1;
    while i < j {
        let addition = arr[i] + arr[j];
        if addition == *sum {
            return Some((i, j));
        } else if addition < *sum {
            i += 1;
        } else {
            j += 1;
        }
    }
    None
}
