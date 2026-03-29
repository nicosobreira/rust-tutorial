pub fn find(array: &[i32]) -> (&[i32], i32) {
    if array.len() == 1 {
        return (array, array[0]);
    }

    let middle = array.len() / 2;

    let (left_sub, left_sum) = find(&array[..middle]);
    let (right_sub, right_sum) = find(&array[middle..]);
    let (cross_sub, cross_sum) = find_max_crossing_subarray(&array[..], middle);

    // Escolhe o subarray com maior soma
    [
        (left_sub, left_sum),
        (right_sub, right_sum),
        (cross_sub, cross_sum),
    ]
    .into_iter()
    .max_by_key(|&(_, sum)| sum)
    .unwrap()
}

fn find_max_crossing_subarray(array: &[i32], middle: usize) -> (&[i32], i32) {
    let left = &array[0..middle];
    let mut sum = 0;
    let mut left_sum = i32::MIN;
    let mut left_index = 0;

    for (i, n) in left.iter().rev().enumerate() {
        let new_sum = sum + n;

        if new_sum > sum {
            left_index = i;
            left_sum = new_sum;
        }

        sum = new_sum;
    }

    let right = &array[middle..];
    let mut sum = 0;
    let mut right_sum = i32::MIN;
    let mut right_index = 0;

    for (i, n) in right.iter().enumerate() {
        let new_sum = sum + n;

        if new_sum > sum {
            right_index = i;
            right_sum = new_sum;
        }

        sum = new_sum;
    }

    let right_index = right_index + middle;

    (&array[left_index..right_index], left_sum + right_sum)
}

#[cfg(test)]
mod tests {
    use super::max_subarray;

    #[test]
    fn test_single_element() {
        let array = [42];
        let (subarray, sum) = max_subarray::find(&array);
        assert_eq!(subarray, &[42]);
        assert_eq!(sum, 42);
    }

    #[test]
    fn test_all_positive() {
        let array = [1, 2, 3, 4, 5];
        let (subarray, sum) = max_subarray::find(&array);
        assert_eq!(subarray, &[1, 2, 3, 4, 5]);
        assert_eq!(sum, 15);
    }

    #[test]
    fn test_mixed_values() {
        let array = [-2, 1, -3, 4, -1, 2, 1, -5, 4];
        let (subarray, sum) = max_subarray::find(&array);
        assert_eq!(subarray, &[4, -1, 2, 1]);
        assert_eq!(sum, 6);
    }

    #[test]
    fn test_all_negative() {
        let array = [-8, -3, -6, -2, -5, -4];
        let (subarray, sum) = max_subarray::find(&array);
        assert_eq!(subarray, &[-2]);
        assert_eq!(sum, -2);
    }

    #[test]
    fn test_multiple_max_subarrays() {
        let array = [1, -2, 3, 3, -2, 3];
        let (subarray, sum) = max_subarray::find(&array);
        // O subarray máximo pode ser [3,3] ou [3,3,-2,3], depende da implementação
        assert_eq!(sum, 6);
        assert_eq!(subarray, &[3, 3]);
    }
}
