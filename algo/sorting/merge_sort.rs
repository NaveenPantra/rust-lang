use std::fmt::Debug;

fn main() {
    let mut arr: [i32; 10] = [9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
    println!("Unsorted array {:?}", arr);
    let arr_len = arr.len() - 1;
    merge(&mut arr, &0, &arr_len);
    println!("Sorted array {:?}", arr);
    print!("\n--\n");
    let arr1: [i32; 4] = [1, 2, 3, 4];
    let arr2: [i32; 4] = [-1, 0, 100, 200];
    println!("Merge of two arrays {:?}, {:?} with sorted is {:?}", &arr1, &arr2, merge_tow_sorted_arrays(&arr1, &arr2));

}

fn merge<T: Debug + Copy + PartialEq + PartialOrd>(arr: &mut [T], start: &usize, end: &usize) {
    if start < end {
        let mid = (start + end) / 2;
        merge(arr, start, &mid);
        merge(arr, &(mid + 1), end);
        merge_sort(arr, *start, mid, *end);
    }
}

fn merge_sort<T: Debug + Copy + PartialEq + PartialOrd>(arr: &mut [T], start: usize, mid: usize, end: usize) {
    let left_arr_len = mid - start + 1;
    let right_arr_len = end - mid;
    let mut left_arr: Vec<T> = Vec::new();
    let mut k = start;
    for _ in 0..left_arr_len {
        left_arr.push(arr[k]);
        k += 1;
    }
    let mut right_arr: Vec<T> = Vec::new();
    for _ in 0..right_arr_len {
        right_arr.push(arr[k]);
        k += 1;
    }
    let mut i: usize = 0;
    let mut j: usize = 0;
    let mut k = start;
    while i < left_arr_len && j < right_arr_len {
        if left_arr[i] <= right_arr[j] {
            arr[k] = left_arr[i];
            i += 1;
        } else {
            arr[k] = right_arr[j];
            j += 1;
        }
        k += 1;
    }
    while i < left_arr_len {
        arr[k] = left_arr[i];
        i += 1;
        k += 1;
    }

    while j < right_arr_len {
        arr[k] = right_arr[j];
        j += 1;
        k += 1;
    }
}


fn merge_tow_sorted_arrays<'a, T: PartialEq + PartialOrd>(arr1: &'a [T], arr2: &'a [T]) -> Vec<&'a T> {
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
