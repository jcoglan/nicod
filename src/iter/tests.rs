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

#[test]
fn interleave_iterator_with_empty() {
    let streams = vec![
        Interleave::new(vec![vec![1, 2, 3, 4]]),
        Interleave::new(vec![] as Vec<Vec<_>>),
    ];

    let combined: Vec<_> = Interleave::new(streams).collect();
    assert_eq!(combined, vec![1, 2, 3, 4]);
}

#[test]
fn interleave_infinite_streams() {
    let streams = vec![0.., 100.., 200..];

    let combined: Vec<_> = Interleave::new(streams).take(9).collect();
    assert_eq!(combined, vec![0, 100, 200, 1, 101, 201, 2, 102, 202]);
}
