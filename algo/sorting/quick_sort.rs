use std::fmt::{Display, Debug};

fn main() {
    let mut arr: [i32; 10] = [9, 8, -1, 100, -100, 20, 30, 23, 0, 1000];
    println!("Unsorted array: {:?}", arr);
    let arr_len = arr.len() - 1;
    quick(&mut arr, &0, &arr_len);
    println!("Sorted array: {:?}", arr);
}

fn quick<T: PartialOrd + PartialEq + Copy + Display + Debug>(arr: &mut [T], start: &usize, end: &usize, ) {
    if start < end {
        let p = partition(arr, &start, &end);
        // usize -1 will be sub overflow so saturating it to 0
        quick(arr, start, &(p.saturating_sub(1)));
        quick(arr, &(p + 1), end);
    }
}

fn partition<T: PartialEq + PartialOrd + Copy + Display + Debug>(arr: &mut [T], start: &usize, end: &usize) -> usize {
    let pivot = arr[*end];
    let mut i: i32 = *start as i32 - 1;
    for j in *start..*end {
        if arr[j] <= pivot {
            i += 1;
            (arr[i as usize], arr[j]) = (arr[j], arr[i as usize]);
        }
    }
    (arr[(i + 1) as usize], arr[*end]) = (arr[*end], arr[(i + 1) as usize]);
    return (i + 1) as usize;
}
