#![feature(test)]
extern crate test;
use test::Bencher;

use nicod::expr::*;
use nicod::state::*;
use nicod::*;

#[bench]
fn words(bench: &mut Bencher) {
    let expr_a = expr!(wrd(a));
    let expr_b = expr!(wrd(a));

    bench.iter(|| State::new().unify(&expr_a, &expr_b));
}

#[bench]
fn var(bench: &mut Bencher) {
    let expr_a = expr!(wrd(a));
    let expr_b = expr!(var(x));

    bench.iter(|| State::new().unify(&expr_a, &expr_b));
}

#[bench]
fn seqs_with_vars(bench: &mut Bencher) {
    let expr_a = expr!(seq(wrd(a), var(w), wrd(b), var(x)));
    let expr_b = expr!(seq(var(y), wrd(c), var(z), wrd(d)));

    bench.iter(|| State::new().unify(&expr_a, &expr_b));
}

#[bench]
fn nested_seqs(bench: &mut Bencher) {
    let expr_a = expr!(seq(
        seq(wrd(a), wrd(b)),
        wrd(c),
        seq(
            wrd(d),
            seq(
                seq(wrd(e)),
                seq(wrd(f), wrd(g)),
                seq(seq(seq(wrd(h)), wrd(i)), wrd(j))
            )
        )
    ));
    let expr_b = expr!(seq(
        seq(wrd(a), wrd(b)),
        wrd(c),
        seq(
            wrd(d),
            seq(
                seq(wrd(e)),
                seq(wrd(f), wrd(g)),
                seq(seq(seq(wrd(h)), wrd(i)), wrd(j))
            )
        )
    ));

    bench.iter(|| State::new().unify(&expr_a, &expr_b));
}

#[bench]
fn nested_seqs_with_indirect_vars(bench: &mut Bencher) {
    let expr_a = expr!(seq(
        seq(var(z), var(w)),
        var(r),
        seq(
            var(u),
            seq(
                seq(var(r)),
                seq(wrd(f), var(w)),
                seq(seq(seq(var(u)), var(t)), var(x))
            )
        )
    ));
    let expr_b = expr!(seq(
        seq(var(y), var(x)),
        var(s),
        seq(
            var(v),
            seq(
                seq(var(q)),
                seq(var(q), var(v)),
                seq(seq(seq(var(t)), var(s)), var(y))
            )
        )
    ));

    bench.iter(|| State::new().unify(&expr_a, &expr_b));
}
