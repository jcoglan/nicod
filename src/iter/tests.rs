#![cfg(test)]

use super::*;

#[test]
fn flatten_single_iterator() {
    let streams = vec![vec![1, 2, 3, 4]];

    let flattened: Vec<_> = Flatten::new(streams).collect();
    assert_eq!(flattened, vec![1, 2, 3, 4]);
}

#[test]
fn flatten_two_iterators() {
    let streams = vec![vec![1, 2, 3, 4], vec![5, 6, 7]];

    let flattened: Vec<_> = Flatten::new(streams).collect();
    assert_eq!(flattened, vec![1, 5, 2, 6, 3, 7, 4]);
}

#[test]
fn flatten_many_iterators() {
    let streams = vec![vec![1, 2], vec![3], vec![4, 5, 6, 7], vec![8, 9, 10]];

    let flattened: Vec<_> = Flatten::new(streams).collect();
    assert_eq!(flattened, vec![1, 3, 2, 4, 8, 5, 9, 6, 10, 7]);
}

#[test]
fn flatten_infinite_ranges() {
    let streams = vec![0.., 10.., 20.., 30..];

    let flattened: Vec<_> = Flatten::new(streams).take(10).collect();
    assert_eq!(flattened, vec![0, 10, 1, 20, 2, 11, 3, 30, 4, 12]);
}

#[test]
fn flatten_infinite_stream_of_ranges() {
    let streams = (0..).map(|n| (100 * n)..);

    let flattened: Vec<_> = Flatten::new(streams).take(20).collect();
    assert_eq!(
        flattened,
        vec![0, 100, 1, 200, 2, 101, 3, 300, 4, 102, 5, 201, 6, 103, 7, 400, 8, 104, 9, 202]
    );
}

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
