#![cfg(test)]

use super::*;
use crate::*;

fn append_rules() -> RuleSet {
    let mut rules = RuleSet::new();

    //  nil ++ $list = $list

    rules.insert(
        "append-0",
        &expr!(seq(wrd(nil), wrd(plus), var(list), wrd(eq), var(list))),
        &[],
    );

    //          $tail ++ $list = $rest
    //  --------------------------------------
    //  ($head $tail) ++ $list = ($head $rest)

    rules.insert(
        "append-N",
        &expr!(seq(
            seq(var(head), var(tail)),
            wrd(plus),
            var(list),
            wrd(eq),
            seq(var(head), var(rest))
        )),
        &[expr!(seq(
            var(tail),
            wrd(plus),
            var(list),
            wrd(eq),
            var(rest)
        ))],
    );

    rules
}

#[test]
fn derive_from_single_rule() {
    // nil ++ (a nil) = ?

    let query = expr!(seq(
        wrd(nil),
        wrd(plus),
        seq(wrd(a), wrd(nil)),
        wrd(eq),
        var(answer)
    ));

    let results: Vec<_> = append_rules()
        .derive(&query)
        .map(|s| s.resolve(&expr!(var(answer))))
        .collect();

    assert_eq!(results, vec![expr!(seq(wrd(a), wrd(nil)))]);
}

#[test]
fn derive_from_recursive_rule() {
    // (a (b (c nil))) ++ (d (e nil)) = ?

    let query = expr!(seq(
        seq(wrd(a), seq(wrd(b), seq(wrd(c), wrd(nil)))),
        wrd(plus),
        seq(wrd(d), seq(wrd(e), wrd(nil))),
        wrd(eq),
        var(answer)
    ));

    let results: Vec<_> = append_rules()
        .derive(&query)
        .map(|s| s.resolve(&expr!(var(answer))))
        .collect();

    assert_eq!(
        results,
        vec![expr!(seq(
            wrd(a),
            seq(wrd(b), seq(wrd(c), seq(wrd(d), seq(wrd(e), wrd(nil)))))
        ))]
    );
}

#[test]
fn derive_in_reverse() {
    // $x ++ $y = (a (b (c (d nil))))

    let query = expr!(seq(
        var(x),
        wrd(plus),
        var(y),
        wrd(eq),
        seq(wrd(a), seq(wrd(b), seq(wrd(c), seq(wrd(d), wrd(nil)))))
    ));

    let results: Vec<_> = append_rules()
        .derive(&query)
        .map(|s| (s.resolve(&expr!(var(x))), s.resolve(&expr!(var(y)))))
        .collect();

    assert_eq!(
        results,
        vec![
            (
                expr!(wrd(nil)),
                expr!(seq(wrd(a), seq(wrd(b), seq(wrd(c), seq(wrd(d), wrd(nil))))))
            ),
            (
                expr!(seq(wrd(a), wrd(nil))),
                expr!(seq(wrd(b), seq(wrd(c), seq(wrd(d), wrd(nil)))))
            ),
            (
                expr!(seq(wrd(a), seq(wrd(b), wrd(nil)))),
                expr!(seq(wrd(c), seq(wrd(d), wrd(nil))))
            ),
            (
                expr!(seq(wrd(a), seq(wrd(b), seq(wrd(c), wrd(nil))))),
                expr!(seq(wrd(d), wrd(nil)))
            ),
            (
                expr!(seq(wrd(a), seq(wrd(b), seq(wrd(c), seq(wrd(d), wrd(nil)))))),
                expr!(wrd(nil))
            ),
        ]
    );
}

fn type_rules() -> RuleSet {
    let mut rules = RuleSet::new();

    //  nil : List

    rules.insert("type-0", &expr!(seq(wrd(nil), wrd(is), wrd(List))), &[]);

    //      $tail : List
    //  --------------------
    //  ($head $tail) : List

    rules.insert(
        "type-N",
        &expr!(seq(seq(var(head), var(tail)), wrd(is), wrd(List))),
        &[expr!(seq(var(tail), wrd(is), wrd(List)))],
    );

    //  $a : List       $b : List
    //  -------------------------
    //      ($a ++ $b) : List

    rules.insert(
        "type-append",
        &expr!(seq(seq(var(a), wrd(plus), var(b)), wrd(is), wrd(List))),
        &[
            expr!(seq(var(a), wrd(is), wrd(List))),
            expr!(seq(var(b), wrd(is), wrd(List))),
        ],
    );

    rules
}

#[test]
fn inductive_type_check() {
    // (a (b (c nil))) : ?

    let query = expr!(seq(
        seq(wrd(a), seq(wrd(b), seq(wrd(c), wrd(nil)))),
        wrd(is),
        var(answer)
    ));

    let results: Vec<_> = type_rules()
        .derive(&query)
        .map(|s| s.resolve(&expr!(var(answer))))
        .collect();

    assert_eq!(results, vec![expr!(wrd(List))]);
}

#[test]
fn inductive_type_check_with_two_premises() {
    // ((a (b (c nil))) ++ (d (e nil))) : ?

    let query = expr!(seq(
        seq(
            seq(wrd(a), seq(wrd(b), seq(wrd(c), wrd(nil)))),
            wrd(plus),
            seq(wrd(d), seq(wrd(e), wrd(nil)))
        ),
        wrd(is),
        var(answer)
    ));

    let results: Vec<_> = type_rules()
        .derive(&query)
        .map(|s| s.resolve(&expr!(var(answer))))
        .collect();

    assert_eq!(results, vec![expr!(wrd(List))]);
}

fn transitive_rules() -> RuleSet {
    let mut rules = RuleSet::new();

    //  $x <: $x

    rules.insert("S-Refl", &expr!(seq(var(x), wrd(sub), var(x))), &[]);

    //  $x <: $y        $y <: $z
    //  ------------------------
    //          $x <: $z

    rules.insert(
        "S-Trans",
        &expr!(seq(var(x), wrd(sub), var(z))),
        &[
            expr!(seq(var(x), wrd(sub), var(y))),
            expr!(seq(var(y), wrd(sub), var(z))),
        ],
    );

    //  a <: b
    //  b <: c
    //  c <: d

    rules.insert("S-AB", &expr!(seq(wrd(a), wrd(sub), wrd(b))), &[]);
    rules.insert("S-BC", &expr!(seq(wrd(b), wrd(sub), wrd(c))), &[]);
    rules.insert("S-CD", &expr!(seq(wrd(c), wrd(sub), wrd(d))), &[]);

    rules
}

#[test]
fn transitive_relation_with_infinite_derivations() {
    // a <: $y

    let query = expr!(seq(wrd(a), wrd(sub), var(y)));

    let count = transitive_rules()
        .derive(&query)
        .take_while(|s| s.resolve(&expr!(var(y))) != expr!(wrd(d)))
        .count();

    assert_eq!(count, 173);

    let results: Vec<_> = transitive_rules()
        .derive(&query)
        .map(|s| s.resolve(&expr!(var(y))))
        .take(3)
        .collect();

    assert_eq!(results, vec![expr!(wrd(a)), expr!(wrd(a)), expr!(wrd(b))]);
}
