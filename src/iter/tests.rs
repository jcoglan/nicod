#![cfg(test)]

use super::*;

#[test]
fn interleave_single_iterator() {
    let streams = vec![vec![1, 2, 3, 4]];

    let combined: Vec<_> = Interleave::new(streams).collect();
    assert_eq!(combined, vec![1, 2, 3, 4]);
}

#[test]
fn interleave_iterators() {
    let streams = vec![vec![1, 2], vec![3, 4, 5, 6], vec![7, 8, 9]];

    let combined: Vec<_> = Interleave::new(streams).collect();
    assert_eq!(combined, vec![1, 3, 7, 2, 4, 8, 5, 9, 6]);
}

#[test]
fn interleave_empty_iterators() {
    let streams = vec![vec![1, 2], vec![], vec![7, 8, 9]];

    let combined: Vec<_> = Interleave::new(streams).collect();
    assert_eq!(combined, vec![1, 7, 2, 8, 9]);
}
