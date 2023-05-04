use std::io;

fn main() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut input: String = String::new();
    println!("Enter a number to search for: ");
    io::stdin().read_line(&mut input).unwrap();
    let midpoint_index = binary_search(arr, input.trim().parse().unwrap());
    println!("Found IT!!!: {}", midpoint_index);
}

fn binary_search(arr: [i8; 10], val: i8) -> usize {
    let mut lower_bound = 0;
    let mut upper_bound = arr.len() - 1;

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
