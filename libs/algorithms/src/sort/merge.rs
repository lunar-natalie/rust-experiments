pub fn sort(arr: &mut [isize]) {
    fn f(vec: &mut Vec<isize>) {
        if vec.len() > 1 {
            // Split into two partitions at midpoint
            let mid: usize = vec.len() / 2;
            let mut left = vec[0..mid].to_vec();
            let mut right = vec[mid..vec.len()].to_vec();
            f(&mut left);
            f(&mut right);

            // Merge left and right into original vector
            let mut il = 0;
            let mut ir = 0;
            let mut iv = 0;
            while il < left.len() && ir < right.len() {
                if left[il] <= right[ir] {
                    vec[iv] = left[il];
                    il += 1;
                } else {
                    vec[iv] = right[ir];
                    ir += 1;
                }
                iv += 1;
            }
            while il < left.len() {
                vec[iv] = left[il];
                iv += 1;
                il += 1;
            }
            while ir < right.len() {
                vec[iv] = right[ir];
                iv += 1;
                ir += 1;
            }
        }
    }
    let mut vec = arr.to_vec();
    f(&mut vec);
    arr.clone_from_slice(vec.as_slice());
}