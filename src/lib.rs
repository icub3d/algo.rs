// Kadane's algorithm for the maximum subarray problem. See:
// https://en.wikipedia.org/wiki/Maximum_subarray_problem#Kadane's_algorithm.
//
// The brute force solution is O(n²) where as this is linear. The key
// insight is that you can track the largest sum from your current
// point in the for loop by determining if the current number would
// add to the value of the largest maximum. If it doesn't, then it
// wouldn't be included in the largest result and either some
// previously known subarray is larger or another one in the future
// is, but it definitely doesn't include our current value and we
// should reset our current maximum. Think of this as a sort of simple
// way of computing a local maximum and then comparing it to your
// global maximum.
pub fn kadanes_max_subarray<T>(values: &[T]) -> T
where
    T: Copy + Default + Ord + std::ops::Add<Output = T>,
{
    let mut best = values[0];
    let mut cur = values[0];
    for x in values.iter().skip(1) {
        cur = *x + T::default().max(cur);
        best = best.max(cur);
    }

    best
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_kadanes_max_subarray() {
        assert_eq!(
            kadanes_max_subarray(&vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        );
        assert_eq!(kadanes_max_subarray(&vec![1]), 1);
        assert_eq!(kadanes_max_subarray(&vec![5, 4, -1, 7, 8]), 23);
        assert_eq!(kadanes_max_subarray(&vec![-1]), -1);
    }
}
