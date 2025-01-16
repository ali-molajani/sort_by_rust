mod sort;
use sort::sort::insertion_sort;
fn main() {
    let sample = vec![4, 3, 7, 1, 9, 11, 0, 4];
    // here we are clone the vector due to have the original vector
    // is it wasn't needed we should use the mutable refrence `&mut`
    // in the passing and in the function definition
    let ans = insertion_sort(sample.clone());
    println!("the sample array: {:?}", sample);
    println!("the sorted array: {:?}", ans);
}
