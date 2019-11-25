#![feature(test)]
extern crate test;
use test::Bencher;

use nicod::expr::*;
use nicod::lang::RuleSet;
use nicod::*;
use std::rc::Rc;

fn sequence_rules() -> RuleSet {
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

fn list_rules() -> RuleSet {
    let mut rules = RuleSet::new();

    //  [] ++ $list = $list

    rules.insert(
        "append-0",
        &expr!(seq(lst(λ, []), wrd(plus), var(list), wrd(eq), var(list))),
        &[],
    );

    //            $tail ++ $list = $rest
    //  ------------------------------------------
    //  [$head | $tail] ++ $list = [$head | $rest]

    rules.insert(
        "append-N",
        &expr!(seq(
            lst(λ, [var(head) | var(tail)]),
            wrd(plus),
            var(list),
            wrd(eq),
            lst(λ, [var(head) | var(rest)])
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

fn word(n: usize) -> Rc<Expr> {
    Rc::new(Expr::Wrd(Word(format!("word-{}", n))))
}

fn gen_sequence(n: usize, term: &str) -> Rc<Expr> {
    let mut expr = Rc::new(Expr::Wrd(Word(String::from(term))));

    for i in (0..n).rev() {
        expr = Rc::new(Expr::Seq(Sequence(vec![word(i), expr])));
    }
    expr
}

fn sequence_append(bench: &mut Bencher, n: usize, term: &str) {
    let rules = sequence_rules();

    let query = Rc::new(Expr::Seq(Sequence(vec![
        gen_sequence(n, term),
        expr!(wrd(plus)),
        gen_sequence(n, "nil"),
        expr!(wrd(eq)),
        expr!(var(answer)),
    ])));

    bench.iter(|| rules.derive(&query).count());
}

#[bench]
fn sequence_append_10(bench: &mut Bencher) {
    sequence_append(bench, 10, "nil");
}

#[bench]
fn sequence_append_fail_10(bench: &mut Bencher) {
    sequence_append(bench, 10, "nope");
}

#[bench]
fn sequence_append_100(bench: &mut Bencher) {
    sequence_append(bench, 100, "nil");
}

#[bench]
fn sequence_append_fail_100(bench: &mut Bencher) {
    sequence_append(bench, 100, "nope");
}

#[bench]
fn sequence_append_1_000(bench: &mut Bencher) {
    sequence_append(bench, 1_000, "nil");
}

#[bench]
fn sequence_append_fail_1_000(bench: &mut Bencher) {
    sequence_append(bench, 1_000, "nope");
}

fn gen_list(n: usize, tag: &str) -> Rc<Expr> {
    let tag = String::from(tag);

    let mut tail = Rc::new(Expr::Lst(List {
        tag: tag.clone(),
        pair: None,
    }));

    for i in (0..n).rev() {
        let tag = tag.clone();
        let head = word(i);
        let pair = Some(Pair { head, tail });

        tail = Rc::new(Expr::Lst(List { tag, pair }));
    }
    tail
}

fn list_append(bench: &mut Bencher, n: usize, tag: &str) {
    let rules = list_rules();

    let query = Rc::new(Expr::Seq(Sequence(vec![
        gen_list(n, tag),
        expr!(wrd(plus)),
        gen_list(n, "λ"),
        expr!(wrd(eq)),
        expr!(var(answer)),
    ])));

    bench.iter(|| rules.derive(&query).count());
}

#[bench]
fn list_append_10(bench: &mut Bencher) {
    list_append(bench, 10, "λ");
}

#[bench]
fn list_append_fail_10(bench: &mut Bencher) {
    list_append(bench, 10, "Λ");
}

#[bench]
fn list_append_100(bench: &mut Bencher) {
    list_append(bench, 100, "λ");
}

#[bench]
fn list_append_fail_100(bench: &mut Bencher) {
    list_append(bench, 100, "Λ");
}

#[bench]
fn list_append_1_000(bench: &mut Bencher) {
    list_append(bench, 1_000, "λ");
}

#[bench]
fn list_append_fail_1_000(bench: &mut Bencher) {
    list_append(bench, 1_000, "Λ");
}
