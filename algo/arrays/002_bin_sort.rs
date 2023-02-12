fn main() {
    let arr: &mut [u8; 14] = &mut [0, 1, 1, 1, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0];
    println!("Unsorted binary array {arr:?}");
    bin_sort(arr);
    println!("Sorted binary array {arr:?}");

}


fn bin_sort(arr: &mut [u8]) {
    let mut start = 0;
    let mut end = arr.len() - 1;
    while start < end {
        if arr[start] > arr[end] {
            swap(arr, start, end);
            start += 1;
        } else if arr[start] == 1 && arr[end] == 1 {
            end -= 1;
        } else if arr[start] == 0 && arr[end] == 0 {
            start += 1;
        } else {
            start += 1;
            end -= 1;
        }
    }
}

fn swap(arr: &mut [u8], i: usize, j: usize) {
    (arr[i], arr[j]) = (arr[j], arr[i]);
}


