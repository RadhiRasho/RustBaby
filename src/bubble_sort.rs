// Desc: Sort an array using bubble sort
// Time complexity: O(n^2) worst case
// Time Complexity: O(n) best case
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        // generate more test cases
        for _ in 0..100 {
            let mut arr = [0; 100];
            for i in 0..100 {
                arr[i] = rand::random::<i8>();
            }
            bubble_sort(&mut arr);
            for i in 0..99 {
                assert!(arr[i] <= arr[i + 1]);
            }
        }
    }
}
