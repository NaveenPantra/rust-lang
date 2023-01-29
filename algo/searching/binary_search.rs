

fn main() {
    let arr: [i32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9,];
    let search_ele: i32 = 18;
    let search_index: Option<usize> = binary_search(&arr, &search_ele);
    match search_index {
        Some(index) => println!("Element {} found ar index {}", &search_ele, index),
        None => println!("{} not found in the array", &search_ele)
    }
}

fn binary_search<T: PartialOrd + PartialEq>(arr: &[T], ele: &T) -> Option<usize> {
    let mut start: usize = 0;
    let mut end: usize = arr.len();
    while start < end {
        let mid: usize = ((start + end) / 2) as usize;
        if arr[mid] == *ele {
            return Some(mid);
        } else if *ele < arr[mid] {
            end = mid - 1;
        } else {
            start = mid + 1;
        }
    }
    None
}
