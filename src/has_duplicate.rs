pub fn has_duplicate_ultra_slow(arr: &[i8]) -> bool {
    for i in 0..arr.len() {
        for j in 0..arr.len() {
            if i != j && arr[i] == arr[j] {
                return true;
            }
        }
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_duplicate_ultra_slow() {
        let mut arr = vec![4, 2, 7, 1, 3];
        assert_eq!(has_duplicate_ultra_slow(&arr), false);
        arr = vec![4, 2, 7, 1, 3, 4];
        assert_eq!(has_duplicate_ultra_slow(&arr), true);
    }
}
