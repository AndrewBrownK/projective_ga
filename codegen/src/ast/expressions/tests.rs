use tracing::Level;
use super::*;
#[test]
fn test_keep_all_elements() {
    let mut vec1 = vec![1, 2, 3, 4];
    let mut vec2 = vec![2, 3, 5, 6];
    let retainer = OrderedConcurrentScan::new([&mut vec1, &mut vec2]);
    retainer.retain_mut(|_arr| true);
    // Shallow_eq sets: [2,2], [3,3]
    // Expected: Keep all: 1,4 (no counterparts), 2,3 (f true) in vec1;
    //           5,6 (no counterparts), 2,3 (f true) in vec2
    assert_eq!(vec1, vec![1, 2, 3, 4]);
    assert_eq!(vec2, vec![2, 3, 5, 6]);
}

#[test]
fn test_keep_only_some_shallow_eq() {
    let mut vec1 = vec![1, 2, 3, 4];
    let mut vec2 = vec![2, 3, 5, 6];
    let retainer = OrderedConcurrentScan::new([&mut vec1, &mut vec2]);
    retainer.retain_mut(|arr| *arr[0] == 3);
    // Shallow_eq sets: [2,2], [3,3]
    // Expected: Keep 1,4 (no counterparts), 3 (f true), remove 2 (f false) in vec1;
    //           5,6 (no counterparts), 3 (f true), remove 2 (f false) in vec2
    assert_eq!(vec1, vec![1, 3, 4]);
    assert_eq!(vec2, vec![3, 5, 6]);
}

#[test]
fn test_remove_all_shallow_eq() {
    let mut vec1 = vec![1, 2, 3, 4];
    let mut vec2 = vec![2, 3, 5, 6];
    let retainer = OrderedConcurrentScan::new([&mut vec1, &mut vec2]);
    retainer.retain_mut(|_arr| false);
    // Shallow_eq sets: [2,2], [3,3]
    // Expected: Keep 1,4 (no counterparts), remove 2,3 (f false) in vec1;
    //           5,6 (no counterparts), remove 2,3 (f false) in vec2
    assert_eq!(vec1, vec![1, 4]);
    assert_eq!(vec2, vec![5, 6]);
}

#[test]
fn test_duplicates_in_shallow_eq() {
    let mut vec1 = vec![1, 2, 2, 3];
    let mut vec2 = vec![2, 2, 4];
    let retainer = OrderedConcurrentScan::new([&mut vec1, &mut vec2]);
    retainer.retain_mut(|_arr| false);
    // Shallow_eq sets: [2,2], [2,2]
    // Expected: Keep 1,3 (no counterparts), remove 2,2 (f false) in vec1;
    //           4 (no counterpart), remove 2,2 (f false) in vec2
    assert_eq!(vec1, vec![1, 3]);
    assert_eq!(vec2, vec![4]);
}

#[test]
fn test_order_fixing() {
    let mut vec1 = vec![5, 1, 2, 3, 4];
    let mut vec2 = vec![2, 3];
    let retainer = OrderedConcurrentScan::new([&mut vec1, &mut vec2]);
    retainer.retain_mut(|arr| *arr[0] == 3);
    // Shallow_eq sets: [2,2], [3,3]
    // Expected: Keep 5,1,4 (no counterparts), 3 (f true), remove 2 (f false) in vec1;
    //           3 (f true), remove 2 (f false) in vec2
    assert_eq!(vec1, vec![1, 3, 4, 5]);
    assert_eq!(vec2, vec![3]);
}