mod max_subarray;

fn main() {
    let array = [-1, 3, 4, -5, 9, -2];

    let max = max_subarray::find(&array[..]);

    println!("{max:?}");
}
