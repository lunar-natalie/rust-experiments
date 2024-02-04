use std::fmt::Debug;

use algorithms::sort;

const DATA_I32: ([i32; 8], [i32; 8]) = (
    [4, 2, 56, -16, 1, 0, 8, 9],
    [-16, 0, 1, 2, 4, 8, 9, 56]);

const DATA_F32: ([f32; 8], [f32; 8]) = (
    [4.66, 56.1, 5.024, 1.0, -16.001, 0.0, 77.6, 9.0],
    [-16.001, 0.0, 1.0, 4.66, 5.024, 9.0, 56.1, 77.6]);

fn test_sort<Array, F>(data: (Array, Array), f: F) where Array: Clone + PartialEq + Debug, F: Fn(&mut Array) {
    let mut result = data.0.clone();
    f(&mut result);
    assert_eq!(result, data.1);
}

#[test]
fn test_bubble() {
    test_sort(DATA_I32, |a| sort::bubble(a));
    test_sort(DATA_F32, |a| sort::bubble(a));
}

#[test]
fn test_insertion() {
    test_sort(DATA_I32, |a| sort::insertion(a));
    test_sort(DATA_F32, |a| sort::insertion(a));
}

#[test]
fn test_merge() {
    test_sort(DATA_I32, |a| sort::merge(a));
    test_sort(DATA_F32, |a| sort::merge(a));
}

#[test]
fn test_quick() {
    test_sort(DATA_I32, |a| sort::quick(a));
    test_sort(DATA_F32, |a| sort::quick(a));
}