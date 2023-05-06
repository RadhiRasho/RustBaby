pub fn binary_search(arr: &[i8], val: i8) -> usize {
    let mut lower_bound = 0;
    let mut upper_bound = arr.len() - 1;

    if (val < arr[lower_bound]) || (val > arr[upper_bound]) {
        return 0;
    }

    while lower_bound <= upper_bound {
        let mid_point: usize = ((lower_bound + upper_bound) / 2) as usize;

        let value_at_mid_point = arr[mid_point];

        if val == value_at_mid_point {
            return mid_point;
        } else if val < value_at_mid_point {
            upper_bound = mid_point - 1;
        } else if val > value_at_mid_point {
            lower_bound = mid_point + 1;
        }
    }
    return 0;
}

// This is a test module
#[cfg(test)]
mod tests {
    // We import the function from the outer scope
    use super::*;

    // This is a test function
    #[test]
    fn test_binary_search() {
        // We create a sorted array to test the function
        let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9];

        // We assert that the function returns the correct index for different values
        assert_eq!(binary_search(&arr, 1), 0); // first element
        assert_eq!(binary_search(&arr, 5), 4); // middle element
        assert_eq!(binary_search(&arr, 9), 8); // last element
        assert_eq!(binary_search(&arr, 6), 5); // arbitrary element

        // We assert that the function returns zero for values not in the array
        assert_eq!(binary_search(&arr, -1), 0);
        assert_eq!(binary_search(&arr, 10), 0);
    }
}
