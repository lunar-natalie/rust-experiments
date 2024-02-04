use crate::Sortable;

pub fn sort(arr: &mut [impl Sortable]) {
    fn f(arr: &mut [impl Sortable], left: usize, right: usize) {
        if left < right {
            let pivot = (|mut left: usize, mut right: usize| -> usize {
                let pivot = arr[left];
                while left < right {
                    while arr[left] < pivot {
                        left += 1;
                    }
                    while arr[right] > pivot {
                        right -= 1;
                    }
                    arr.swap(left, right);
                }
                return left;
            })(left, right);
            f(arr, left, pivot - 1);
            f(arr, pivot + 1, right);
        }
    }
    f(arr, 0, arr.len() - 1);
}
