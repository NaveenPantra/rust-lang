fn main() {
    let mut arr: [i32; 10] = [9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
    println!("Unsorted array {:?}", arr);
    merge(&mut arr);
    println!("Sorted array {:?}", arr);
    println!("{:?}", merge_sort_new_arr(&[1, 2, 3, 4], &[-1, 0, 100, 200]));

}

fn merge<T>(arr: &mut [T]) {}

fn merge_sort<T>(arr1: &mut [T], arr2: &mut [T]) {
}


fn merge_sort_new_arr<'a, T: PartialEq + PartialOrd>(arr1: &'a [T], arr2: &'a [T]) -> Vec<&'a T> {
    let mut i: usize = 0;
    let mut j: usize = 0;
    let mut sorted_vec  = Vec::<&T>::new();
    while i < arr1.len() && j < arr2.len() {
        if arr1[i] <= arr2[j] {
            sorted_vec.push(&arr1[i]);
            i += 1;
        } else {
            sorted_vec.push(&arr2[j]);
            j += 1;
        }
    }
    while i < arr1.len() {
        sorted_vec.push(&arr1[i]);
        i += 1;
    }
    while j < arr2.len() {
        sorted_vec.push(&arr2[j]);
        j += 1;
    }
    sorted_vec
}
