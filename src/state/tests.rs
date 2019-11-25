#![cfg(test)]

use crate::expr::*;
use crate::state::*;
use crate::*;

#[test]
fn unify_equal_words() {
    let state = unify!(wrd(a), wrd(a));
    assert!(state.is_some());
    assert!(state.unwrap().values.is_empty());
}

#[test]
fn reject_unequal_words() {
    let state = unify!(wrd(a), wrd(b));
    assert!(state.is_none());
}

#[test]
fn unify_variable_with_word() {
    let state = unify!(var(x), wrd(a)).unwrap();
    assert_eq!(state.resolve(&expr!(var(x))), expr!(wrd(a)));
}

#[test]
fn unify_word_with_variable() {
    let state = unify!(wrd(b), var(y)).unwrap();
    assert_eq!(state.resolve(&expr!(var(y))), expr!(wrd(b)));
}

#[test]
fn unify_variable_with_different_variable() {
    let state = unify!(var(x), var(y)).unwrap();
    assert_eq!(state.resolve(&expr!(var(x))), expr!(var(y)));
    assert_eq!(state.resolve(&expr!(var(y))), expr!(var(y)));
}

#[test]
fn unify_variable_with_same_variable() {
    let state = unify!(var(x), var(x)).unwrap();
    assert_eq!(state.resolve(&expr!(var(x))), expr!(var(x)));
}

#[test]
fn unify_equal_flat_sequences() {
    let state = unify!(seq(wrd(a), wrd(b)), seq(wrd(a), wrd(b)));
    assert!(state.is_some());
    assert!(state.unwrap().values.is_empty());
}

#[test]
fn reject_shorter_sequences() {
    let state = unify!(seq(wrd(a), wrd(b)), seq(wrd(a)));
    assert!(state.is_none());
}

#[test]
fn reject_longer_sequences() {
    let state = unify!(seq(wrd(a)), seq(wrd(a), wrd(b)));
    assert!(state.is_none());
}

#[test]
fn reject_unequal_flat_sequences() {
    let state = unify!(seq(wrd(a), wrd(b)), seq(wrd(a), wrd(c)));
    assert!(state.is_none());
}

#[test]
fn unify_sequences_with_variables() {
    let state = unify!(seq(wrd(a), var(x)), seq(var(y), wrd(b))).unwrap();
    assert_eq!(state.resolve(&expr!(var(x))), expr!(wrd(b)));
    assert_eq!(state.resolve(&expr!(var(y))), expr!(wrd(a)));
}

#[test]
fn unify_sequence_with_repeated_variable() {
    let state = unify!(seq(var(x), wrd(a)), seq(wrd(a), var(x))).unwrap();
    assert_eq!(state.resolve(&expr!(var(x))), expr!(wrd(a)));
}

#[test]
fn reject_sequence_with_repeated_variable() {
    let state = unify!(seq(var(x), wrd(b)), seq(wrd(a), var(x)));
    assert!(state.is_none());
}

#[test]
fn unify_nested_sequences() {
    let state = unify!(
        seq(
            seq(wrd(a), wrd(b)),
            wrd(c),
            seq(wrd(d), seq(wrd(e), wrd(f)))
        ),
        seq(
            seq(wrd(a), wrd(b)),
            wrd(c),
            seq(wrd(d), seq(wrd(e), wrd(f)))
        )
    );

    assert!(state.is_some());
}

#[test]
fn reject_unequal_nested_sequences() {
    let state = unify!(seq(wrd(a), seq(wrd(b), wrd(c))), seq(wrd(a), seq(wrd(b))));
    assert!(state.is_none());
}

#[test]
fn unify_repeat_indirect_vars() {
    let state = unify!(seq(var(x), wrd(a), var(x)), seq(var(y), var(y), wrd(a))).unwrap();
    assert_eq!(state.resolve(&expr!(var(x))), expr!(wrd(a)));
    assert_eq!(state.resolve(&expr!(var(y))), expr!(wrd(a)));
}

#[test]
fn reject_repeat_indirect_vars() {
    let state = unify!(seq(var(x), wrd(a), var(x)), seq(var(y), var(y), wrd(b)));
    assert!(state.is_none());
}

#[test]
fn unify_nested_sequences_with_variables() {
    let state = unify!(
        seq(seq(var(w), wrd(b)), wrd(c), seq(wrd(d), var(x))),
        seq(
            seq(wrd(a), var(y)),
            var(z),
            seq(wrd(d), seq(wrd(e), wrd(f)))
        )
    )
    .unwrap();

    assert_eq!(state.resolve(&expr!(var(w))), expr!(wrd(a)));
    assert_eq!(state.resolve(&expr!(var(x))), expr!(seq(wrd(e), wrd(f))));
    assert_eq!(state.resolve(&expr!(var(y))), expr!(wrd(b)));
    assert_eq!(state.resolve(&expr!(var(z))), expr!(wrd(c)));
}

#[test]
fn unify_partial_pairs() {
    let state = unify!(
        seq(seq(var(y), wrd(b)), var(x)),
        seq(var(x), seq(wrd(a), var(z)))
    )
    .unwrap();

    assert_eq!(state.resolve(&expr!(var(x))), expr!(seq(wrd(a), wrd(b))));
    assert_eq!(state.resolve(&expr!(var(y))), expr!(wrd(a)));
    assert_eq!(state.resolve(&expr!(var(z))), expr!(wrd(b)));
}

#[test]
fn reject_partial_pairs() {
    let state = unify!(
        seq(seq(var(y), wrd(b)), var(x)),
        seq(var(x), seq(wrd(a), var(y)))
    );

    assert!(state.is_none());
}

#[test]
fn unify_nested_sequences_with_indirect_variables() {
    let state = unify!(
        seq(
            seq(wrd(a), var(w)),
            var(x),
            seq(wrd(d), seq(var(z), wrd(f)))
        ),
        seq(
            seq(wrd(a), var(x)),
            wrd(c),
            seq(wrd(d), seq(var(y), var(z)))
        )
    )
    .unwrap();

    assert_eq!(state.resolve(&expr!(var(w))), expr!(wrd(c)));
    assert_eq!(state.resolve(&expr!(var(x))), expr!(wrd(c)));
    assert_eq!(state.resolve(&expr!(var(y))), expr!(wrd(f)));
    assert_eq!(state.resolve(&expr!(var(z))), expr!(wrd(f)));
}

#[test]
fn unify_nested_sequences_with_repeated_variables() {
    let state = unify!(
        seq(
            seq(wrd(a), var(x)),
            wrd(k),
            seq(wrd(d), seq(wrd(e), var(x)))
        ),
        seq(
            seq(wrd(a), wrd(k)),
            var(x),
            seq(wrd(d), seq(wrd(e), wrd(k)))
        )
    )
    .unwrap();

    assert_eq!(state.resolve(&expr!(var(x))), expr!(wrd(k)));
}

#[test]
fn unify_empty_lists() {
    let state = unify!(lst(a, []), lst(a, []));
    assert!(state.is_some());
}

#[test]
fn reject_lists_with_different_tags() {
    let state = unify!(lst(a, []), lst(b, []));
    assert!(state.is_none());
}

#[test]
fn unify_lists_with_equal_elements() {
    let state = unify!(lst([wrd(a), wrd(b),]), lst([wrd(a), wrd(b),]));
    assert!(state.is_some());
}

#[test]
fn unify_lists_with_variables() {
    let state = unify!(lst([var(x), wrd(b),]), lst([wrd(a), var(y),])).unwrap();
    assert_eq!(state.resolve(&expr!(var(x))), expr!(wrd(a)));
    assert_eq!(state.resolve(&expr!(var(y))), expr!(wrd(b)));
}

#[test]
fn reject_shorter_lists() {
    let state = unify!(lst([wrd(a), wrd(b),]), lst([wrd(a),]));
    assert!(state.is_none());
}

#[test]
fn reject_longer_lists() {
    let state = unify!(lst([wrd(a),]), lst([wrd(a), wrd(b),]));
    assert!(state.is_none());
}

#[test]
fn reject_empty_and_nonempty_lists() {
    let state = unify!(lst([]), lst([wrd(a),]));
    assert!(state.is_none());
}

#[test]
fn reject_nonempty_and_empty_lists() {
    let state = unify!(lst([wrd(a),]), lst([]));
    assert!(state.is_none());
}

#[test]
fn unify_list_with_tail_variable() {
    let state = unify!(
        lst(k, [wrd(a), wrd(b) | var(x)]),
        lst(k, [wrd(a), wrd(b), wrd(c), wrd(d),])
    )
    .unwrap();

    assert_eq!(
        state.resolve(&expr!(var(x))),
        expr!(lst(k, [wrd(c), wrd(d),]))
    );
}
