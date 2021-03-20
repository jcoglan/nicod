#![cfg(test)]

use crate::expr::*;
use crate::state::*;
use crate::*;

fn unify<'e>(a: &'e Expr, b: &'e Expr) -> Option<State<'e>> {
    State::new().unify((0, a), (0, b))
}

#[test]
fn unify_equal_words() {
    let x = expr!(wrd(a));

    let state = unify(&x, &x);
    assert!(state.is_some());
    assert!(state.unwrap().values.is_empty());
}

#[test]
fn reject_unequal_words() {
    let x = expr!(wrd(a));
    let y = expr!(wrd(b));

    let state = unify(&x, &y);
    assert!(state.is_none());
}

#[test]
fn unify_variable_with_word() {
    let x = expr!(var(x));
    let y = expr!(wrd(a));

    let state = unify(&x, &y).unwrap();
    assert_eq!(state.resolve(&expr!(var(x))), expr!(wrd(a)));
}

#[test]
fn unify_word_with_variable() {
    let x = expr!(wrd(b));
    let y = expr!(var(y));

    let state = unify(&x, &y).unwrap();
    assert_eq!(state.resolve(&expr!(var(y))), expr!(wrd(b)));
}

#[test]
fn unify_variable_with_different_variable() {
    let x = expr!(var(x));
    let y = expr!(var(y));

    let state = unify(&x, &y).unwrap();
    assert_eq!(state.resolve(&expr!(var(x))), expr!(var(y)));
    assert_eq!(state.resolve(&expr!(var(y))), expr!(var(y)));
}

#[test]
fn unify_variable_with_same_variable() {
    let x = expr!(var(x));
    let y = expr!(var(x));

    let state = unify(&x, &y).unwrap();
    assert_eq!(state.resolve(&expr!(var(x))), expr!(var(x)));
}

#[test]
fn unify_equal_flat_sequences() {
    let x = expr!(seq(wrd(a), wrd(b)));
    let y = expr!(seq(wrd(a), wrd(b)));

    let state = unify(&x, &y);
    assert!(state.is_some());
    assert!(state.unwrap().values.is_empty());
}

#[test]
fn reject_shorter_sequences() {
    let x = expr!(seq(wrd(a), wrd(b)));
    let y = expr!(seq(wrd(a)));

    let state = unify(&x, &y);
    assert!(state.is_none());
}

#[test]
fn reject_longer_sequences() {
    let x = expr!(seq(wrd(a)));
    let y = expr!(seq(wrd(a), wrd(b)));

    let state = unify(&x, &y);
    assert!(state.is_none());
}

#[test]
fn reject_unequal_flat_sequences() {
    let x = expr!(seq(wrd(a), wrd(b)));
    let y = expr!(seq(wrd(a), wrd(c)));

    let state = unify(&x, &y);
    assert!(state.is_none());
}

#[test]
fn unify_sequences_with_variables() {
    let x = expr!(seq(wrd(a), var(x)));
    let y = expr!(seq(var(y), wrd(b)));

    let state = unify(&x, &y).unwrap();
    assert_eq!(state.resolve(&expr!(var(x))), expr!(wrd(b)));
    assert_eq!(state.resolve(&expr!(var(y))), expr!(wrd(a)));
}

#[test]
fn unify_sequence_with_repeated_variable() {
    let x = expr!(seq(var(x), wrd(a)));
    let y = expr!(seq(wrd(a), var(x)));

    let state = unify(&x, &y).unwrap();
    assert_eq!(state.resolve(&expr!(var(x))), expr!(wrd(a)));
}

#[test]
fn reject_sequence_with_repeated_variable() {
    let x = expr!(seq(var(x), wrd(b)));
    let y = expr!(seq(wrd(a), var(x)));

    let state = unify(&x, &y);
    assert!(state.is_none());
}

#[test]
fn unify_nested_sequences() {
    let x = expr!(seq(
        seq(wrd(a), wrd(b)),
        wrd(c),
        seq(wrd(d), seq(wrd(e), wrd(f)))
    ));
    let y = expr!(seq(
        seq(wrd(a), wrd(b)),
        wrd(c),
        seq(wrd(d), seq(wrd(e), wrd(f)))
    ));

    let state = unify(&x, &y);
    assert!(state.is_some());
}

#[test]
fn reject_unequal_nested_sequences() {
    let x = expr!(seq(wrd(a), seq(wrd(b), wrd(c))));
    let y = expr!(seq(wrd(a), seq(wrd(b))));

    let state = unify(&x, &y);
    assert!(state.is_none());
}

#[test]
fn unify_repeat_indirect_vars() {
    let x = expr!(seq(var(x), wrd(a), var(x)));
    let y = expr!(seq(var(y), var(y), wrd(a)));

    let state = unify(&x, &y).unwrap();
    assert_eq!(state.resolve(&expr!(var(x))), expr!(wrd(a)));
    assert_eq!(state.resolve(&expr!(var(y))), expr!(wrd(a)));
}

#[test]
fn reject_repeat_indirect_vars() {
    let x = expr!(seq(var(x), wrd(a), var(x)));
    let y = expr!(seq(var(y), var(y), wrd(b)));

    let state = unify(&x, &y);
    assert!(state.is_none());
}

#[test]
fn unify_nested_sequences_with_variables() {
    let x = expr!(seq(seq(var(w), wrd(b)), wrd(c), seq(wrd(d), var(x))));
    let y = expr!(seq(
        seq(wrd(a), var(y)),
        var(z),
        seq(wrd(d), seq(wrd(e), wrd(f)))
    ));

    let state = unify(&x, &y).unwrap();
    assert_eq!(state.resolve(&expr!(var(w))), expr!(wrd(a)));
    assert_eq!(state.resolve(&expr!(var(x))), expr!(seq(wrd(e), wrd(f))));
    assert_eq!(state.resolve(&expr!(var(y))), expr!(wrd(b)));
    assert_eq!(state.resolve(&expr!(var(z))), expr!(wrd(c)));
}

#[test]
fn unify_partial_pairs() {
    let x = expr!(seq(seq(var(y), wrd(b)), var(x)));
    let y = expr!(seq(var(x), seq(wrd(a), var(z))));

    let state = unify(&x, &y).unwrap();
    assert_eq!(state.resolve(&expr!(var(x))), expr!(seq(wrd(a), wrd(b))));
    assert_eq!(state.resolve(&expr!(var(y))), expr!(wrd(a)));
    assert_eq!(state.resolve(&expr!(var(z))), expr!(wrd(b)));
}

#[test]
fn reject_partial_pairs() {
    let x = expr!(seq(seq(var(y), wrd(b)), var(x)));
    let y = expr!(seq(var(x), seq(wrd(a), var(y))));

    let state = unify(&x, &y);
    assert!(state.is_none());
}

#[test]
fn unify_nested_sequences_with_indirect_variables() {
    let x = expr!(seq(
        seq(wrd(a), var(w)),
        var(x),
        seq(wrd(d), seq(var(z), wrd(f)))
    ));
    let y = expr!(seq(
        seq(wrd(a), var(x)),
        wrd(c),
        seq(wrd(d), seq(var(y), var(z)))
    ));

    let state = unify(&x, &y).unwrap();
    assert_eq!(state.resolve(&expr!(var(w))), expr!(wrd(c)));
    assert_eq!(state.resolve(&expr!(var(x))), expr!(wrd(c)));
    assert_eq!(state.resolve(&expr!(var(y))), expr!(wrd(f)));
    assert_eq!(state.resolve(&expr!(var(z))), expr!(wrd(f)));
}

#[test]
fn unify_nested_sequences_with_repeated_variables() {
    let x = expr!(seq(
        seq(wrd(a), var(x)),
        wrd(k),
        seq(wrd(d), seq(wrd(e), var(x)))
    ));
    let y = expr!(seq(
        seq(wrd(a), wrd(k)),
        var(x),
        seq(wrd(d), seq(wrd(e), wrd(k)))
    ));

    let state = unify(&x, &y).unwrap();
    assert_eq!(state.resolve(&expr!(var(x))), expr!(wrd(k)));
}

#[test]
fn unify_empty_lists() {
    let x = expr!(lst(a, []));
    let y = expr!(lst(a, []));

    let state = unify(&x, &y);
    assert!(state.is_some());
}

#[test]
fn reject_lists_with_different_tags() {
    let x = expr!(lst(a, []));
    let y = expr!(lst(b, []));

    let state = unify(&x, &y);
    assert!(state.is_none());
}

#[test]
fn unify_lists_with_equal_elements() {
    let x = expr!(lst([wrd(a), wrd(b),]));
    let y = expr!(lst([wrd(a), wrd(b),]));

    let state = unify(&x, &y);
    assert!(state.is_some());
}

#[test]
fn unify_lists_with_variables() {
    let x = expr!(lst([var(x), wrd(b),]));
    let y = expr!(lst([wrd(a), var(y),]));

    let state = unify(&x, &y).unwrap();
    assert_eq!(state.resolve(&expr!(var(x))), expr!(wrd(a)));
    assert_eq!(state.resolve(&expr!(var(y))), expr!(wrd(b)));
}

#[test]
fn reject_shorter_lists() {
    let x = expr!(lst([wrd(a), wrd(b),]));
    let y = expr!(lst([wrd(a),]));

    let state = unify(&x, &y);
    assert!(state.is_none());
}

#[test]
fn reject_longer_lists() {
    let x = expr!(lst([wrd(a),]));
    let y = expr!(lst([wrd(a), wrd(b),]));

    let state = unify(&x, &y);
    assert!(state.is_none());
}

#[test]
fn reject_empty_and_nonempty_lists() {
    let x = expr!(lst([]));
    let y = expr!(lst([wrd(a),]));

    let state = unify(&x, &y);
    assert!(state.is_none());
}

#[test]
fn reject_nonempty_and_empty_lists() {
    let x = expr!(lst([wrd(a),]));
    let y = expr!(lst([]));

    let state = unify(&x, &y);
    assert!(state.is_none());
}

#[test]
fn unify_list_with_tail_variable() {
    let x = expr!(lst(k, [wrd(a), wrd(b) | var(x)]));
    let y = expr!(lst(k, [wrd(a), wrd(b), wrd(c), wrd(d),]));

    let state = unify(&x, &y).unwrap();
    assert_eq!(
        state.resolve(&expr!(var(x))),
        expr!(lst(k, [wrd(c), wrd(d),]))
    );
}
