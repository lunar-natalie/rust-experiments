pub fn sort(arr: &mut [isize]) {
    let mut done = false;
    while !done {
        done = true;
        for i in 0..arr.len() - 1 {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                done = false;
            }
        }
    }
}
