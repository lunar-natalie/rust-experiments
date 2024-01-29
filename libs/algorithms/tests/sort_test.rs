use algorithms::sort;

const UNSORTED: [isize; 8] = [4, 2, 56, -1, 1, 0, 8, 9];
const SORTED: [isize; 8] = [-1, 0, 1, 2, 4, 8, 9, 56];

fn test_sort<F>(f: F) where F: Fn(&mut [isize]) {
    let mut a = UNSORTED.clone();
    f(&mut a);
    assert_eq!(a, SORTED);
}

#[test]
fn test_bubble() {
    test_sort(|a| sort::bubble(a));
}

#[test]
fn test_insertion() {
    test_sort(|a| sort::insertion(a));
}

#[test]
fn test_merge() {
    test_sort(|a| sort::merge(a));
}

#[test]
fn test_quick() {
    test_sort(|a| sort::quick(a));
}