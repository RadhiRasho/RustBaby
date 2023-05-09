use std::collections::HashSet;

// Desc: Find if an array has duplicate elements
// Time complexity: O(n^2) worst case
// Time Complexity: O(n) best case
pub fn has_duplicate_ultra_slow(arr: &[i8]) -> bool {
    for i in 0..arr.len() - 1 {
        for j in 0..arr.len() {
            if i != j && arr[i] == arr[j] {
                return true;
            }
        }
    }
    return false;
}
// Desc: Find if an array has duplicate elements using a hash set as a linear search
// Time complexity: O(n) worst case
// Time Complexity: O(1) best case
pub fn has_duplicate_slow(arr: &[i8]) -> bool {
    let mut seen: HashSet<i8> = HashSet::new(); // create an empty hash set

    for &num in arr {
        // iterate over the array elements
        if seen.contains(&num) {
            // check if the current element is already in the set
            return true; // return true if it is a duplicate
        } else {
            seen.insert(num); // insert the current element into the set
        }
    }
    return false; // return false if no duplicates are found
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_duplicate_ultra_slow() {
        let arr = vec![4, 2, 7, 1, 3];
        assert_eq!(has_duplicate_ultra_slow(&arr), false);
        let arr2 = vec![4, 2, 7, 1, 3, 4];
        assert_eq!(has_duplicate_ultra_slow(&arr2), true);
    }

    #[test]
    fn test_has_duplicate_slow() {
        let arr = vec![4, 2, 7, 1, 3, 4];
        assert_eq!(has_duplicate_slow(&arr), true);
        let arr2 = vec![4, 2, 7, 1, 3, 4, 1];
        assert_eq!(has_duplicate_slow(&arr2), true);
    }
}
