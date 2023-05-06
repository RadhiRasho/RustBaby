pub fn bubble_sort(arr: &mut [i8]) {
    let mut sorted: bool = false;
    while !sorted {
        sorted = true;
        for i in 0..arr.len() - 1 {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                sorted = false;
            }
        }
    }
}
