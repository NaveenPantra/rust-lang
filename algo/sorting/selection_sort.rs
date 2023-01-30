fn main() {
    let mut arr: [i32; 10] = [9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
    println!("Unsorted array {:?}", arr);
    selection_sort(&mut arr);
    println!("Sorted array {:?}", arr);
}

fn selection_sort<T: PartialEq + PartialOrd + Copy>(arr: &mut [T]) {
    for i in 0..(arr.len()) {
        let min_index = get_min_ele_ind(&arr[i..]);
        // i + min_index because the slice we are sending above is from i.
        (arr[i], arr[i + min_index]) = (arr[i + min_index], arr[i]);
    }
}

fn get_min_ele_ind<T: PartialEq + PartialOrd + Copy>(arr: &[T]) ->  usize {
    let mut min_ind: usize = 0;
    for i in 1..arr.len() {
        if arr[i] < arr[min_ind] {
            min_ind = i;
        }
    }
    min_ind
}
