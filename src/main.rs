mod bubble_sort;

fn main() {
    // let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    // let mut input: String = String::new();
    // println!("Enter a number to search for: ");
    // io::stdin().read_line(&mut input).unwrap();
    // let midpoint_index = binarysearch::binary_search(arr, input.trim().parse().unwrap());
    // println!("Found IT!!!: {}", midpoint_index);

    let mut arr = vec![4, 2, 7, 1, 3];
    bubble_sort::bubble_sort(&mut arr);
    println!("{:?}", arr);
}
