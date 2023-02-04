
fn main() {
    let arr: &mut [i16] = &mut [8, 9, 10, 2, -1, 100, 10, -200, -200, 23, 243, 23, 0, -0];
    println!("Unsorted array {:?}", arr);
    max_heap_sort(arr);
    // min_heap_sort(arr);
    println!("Sorted array {:?}", arr);
}

fn max_heap_sort<T: PartialOrd + Copy>(arr: &mut [T]) {
    build_max_heap(arr);
    let mut heap_size = arr.len() - 1;
    for element_index in (1..arr.len()).rev() {
        (arr[0], arr[element_index]) = (arr[element_index], arr[0]);
        heap_size -= 1;
        max_heapify(arr, 0, heap_size);
    }
}

fn build_max_heap<T: PartialOrd + Copy>(arr: &mut [T]) {
    let heap_size = arr.len();
    for  element_index in (0..=(heap_size / 2)).rev() {
        max_heapify(arr, element_index, heap_size)
    }
}

fn max_heapify<T: PartialOrd + Copy>(arr: &mut [T], element_index: usize, heap_size: usize) {
    if element_index > heap_size {
        return;
    }
    let mut largest_index = element_index;
    let left_element_index = left_heap_element(element_index);
    let right_element_index = right_heap_element(element_index);
    if left_element_index < heap_size && arr[largest_index] < arr[left_element_index] {
        largest_index = left_element_index;
    }
    if right_element_index < heap_size && arr[largest_index] < arr[right_element_index]  {
        largest_index = right_element_index;
    }
    if element_index != largest_index {
        (arr[element_index], arr[largest_index]) = (arr[largest_index], arr[element_index]);
        max_heapify(arr, largest_index, heap_size);
    }
}

fn min_heap_sort<T: PartialOrd + Copy>(arr: &mut [T]) {
    build_min_heap(arr);
    let mut heap_size = arr.len();
    for element_index in (1..arr.len()).rev() {
        (arr[element_index], arr[0]) = (arr[0], arr[element_index]);
        heap_size -= 1;
        min_heapify(arr, 0, heap_size);
    }
}

fn build_min_heap<T: PartialOrd + Copy>(arr: &mut [T]) {
    let heap_size = arr.len();
    for element_index in (0..=(heap_size / 2)).rev() {
        min_heapify(arr, element_index, heap_size);
    }
}

fn min_heapify<T: PartialOrd + Copy>(arr: &mut [T], element_index: usize, heap_size: usize) {
    if element_index > heap_size {
        return;
    }
    let left_element_index = left_heap_element(element_index);
    let right_element_index = right_heap_element(element_index);
    let mut smallest_element_index = element_index;
    if left_element_index < heap_size && arr[smallest_element_index] > arr[left_element_index] {
        smallest_element_index = left_element_index;
    }
    if right_element_index < heap_size && arr[smallest_element_index] > arr[right_element_index] {
        smallest_element_index = right_element_index;
    }
    if smallest_element_index != element_index {
        (arr[element_index], arr[smallest_element_index]) = (arr[smallest_element_index], arr[element_index]);
        min_heapify(arr, smallest_element_index, heap_size);
    }
}

fn left_heap_element(parent_index: usize) -> usize {
    (2 * parent_index) + 1
}

fn right_heap_element(parent_index: usize) -> usize {
    (2 * parent_index) + 2
}

fn parent_heap_element(child_index: usize) -> usize {
    child_index / 2
}
