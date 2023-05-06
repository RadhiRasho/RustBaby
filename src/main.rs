mod binary_search;
mod bubble_sort;
mod has_duplicate;

fn main() {
    let mut arr = vec![4, 2, 7, 1, 3];
    bubble_sort::bubble_sort(&mut arr);
    println!("{:?}", arr);
}
