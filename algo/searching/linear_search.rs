fn main() {
    let arr: [i32; 10] = [10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
    let search_ele: i32 = 11;
    let index: Option<usize> = linear_search(&arr, &search_ele);
    match index {
        Some(ind) => println!("Element {} found at index {}", &search_ele, ind),
        None=> println!("{} not found in array", &search_ele),
    };
}

fn linear_search<T: PartialEq + PartialOrd>(arr: &[T], ele: &T) -> Option<usize> {
    for i in 0..arr.len() {
        if arr[i] == *ele {
            return Some(i as usize);
        }
    };
    None
}
