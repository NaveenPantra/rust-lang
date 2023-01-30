fn main() {
    let mut arr: [i32; 10] = [9, 8, 7, 6, 5, 4, 3, 2, 1, 0,];
    println!("Unsorted array {:?}", arr);
    insertion_sort(&mut arr);
    println!("Sorted array {:?}", arr);
}

fn insertion_sort<T: PartialEq + PartialOrd + Copy>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let key = arr[i];
        let mut j = (i - 1) as i32;
        while j >= 0 && key < arr[j as usize] {
            arr[j as usize + 1] = arr[j as usize];
            j -= 1;
        }
        arr[(j + 1) as usize] = key;
    }
}
