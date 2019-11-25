#![feature(test)]
extern crate test;
use test::Bencher;

use nicod::expr::*;
use nicod::lang::RuleSet;
use nicod::*;

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
    //  d <: e

    rules.insert("S-AB", &expr!(seq(wrd(a), wrd(sub), wrd(b))), &[]);
    rules.insert("S-BC", &expr!(seq(wrd(b), wrd(sub), wrd(c))), &[]);
    rules.insert("S-CD", &expr!(seq(wrd(c), wrd(sub), wrd(d))), &[]);
    rules.insert("S-DE", &expr!(seq(wrd(d), wrd(sub), wrd(e))), &[]);

    rules
}

fn subtype_search(bench: &mut Bencher, typ: std::rc::Rc<Expr>) {
    let rules = transitive_rules();
    let query = expr!(seq(wrd(a), wrd(sub), var(t)));
    let target = expr!(var(t));

    bench.iter(|| {
        rules.derive(&query).find(|s| s.resolve(&target) == typ);
    });
}

#[bench]
fn breadth_first_b(bench: &mut Bencher) {
    subtype_search(bench, expr!(wrd(b)));
}

#[bench]
fn breadth_first_c(bench: &mut Bencher) {
    subtype_search(bench, expr!(wrd(c)));
}

#[bench]
fn breadth_first_d(bench: &mut Bencher) {
    subtype_search(bench, expr!(wrd(d)));
}

#[bench]
fn breadth_first_e(bench: &mut Bencher) {
    subtype_search(bench, expr!(wrd(e)));
}
