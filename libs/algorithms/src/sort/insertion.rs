pub fn sort(arr: &mut [isize]) {
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

// This is what we'd have to do to adhere to OCR's implementation :/
// pub fn sort(arr: &mut [isize]) {
//     for i in 1..arr.len() {
//         let key = arr[i];
//         let mut end = (i - 1) as isize;
//         while end >= 0 && arr[end as usize] > key {
//             // Shift values left
//             arr[(end + 1) as usize] = arr[end as usize];
//             end -= 1;
//         }
//         arr[(end + 1) as usize] = key;
//     }
// }
