fn main() {
    let mut arr: [i32; 10] = [9, 8, 7, 6, 5, 4, 3, 2, 1, 0,];
    println!("Unsorted array {:?}", arr);
    bubble_sort(&mut arr);
    println!("Sorted array {:?}", arr);
}

fn bubble_sort<T: PartialEq + PartialOrd + Copy>(arr: &mut [T]) {
    for i in 0..arr.len() {
        for j in ((i + 1)..arr.len()).rev() {
            if arr[j] < arr[i] {
                (arr[j], arr[i]) = (arr[i], arr[j]);
            }
        }
    }
}
