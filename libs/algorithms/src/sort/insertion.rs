use crate::Sortable;

pub fn sort(arr: &mut [impl Sortable]) {
    for i in 1..arr.len() {
        let key = arr[i];
        let mut end = i;
        while end > 0 && arr[end - 1] > key {
            // Shift values left
            arr[end] = arr[end - 1];
            end -= 1;
        }
        arr[end] = key;
    }
}