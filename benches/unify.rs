#![feature(test)]
extern crate test;
use test::Bencher;

use nicod::expr::*;
use nicod::state::*;
use nicod::*;

fn unify(a: &Expr, b: &Expr) -> Option<State> {
    State::new().unify((0, a), (0, b))
}

#[bench]
fn words(bench: &mut Bencher) {
    let expr_a = expr!(wrd(a));
    let expr_b = expr!(wrd(a));

    bench.iter(|| unify(&expr_a, &expr_b));
}

#[bench]
fn var(bench: &mut Bencher) {
    let expr_a = expr!(wrd(a));
    let expr_b = expr!(var(x));

    bench.iter(|| unify(&expr_a, &expr_b));
}

#[bench]
fn seqs_with_vars(bench: &mut Bencher) {
    let expr_a = expr!(seq(wrd(a), var(w), wrd(b), var(x)));
    let expr_b = expr!(seq(var(y), wrd(c), var(z), wrd(d)));

    bench.iter(|| unify(&expr_a, &expr_b));
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

    bench.iter(|| unify(&expr_a, &expr_b));
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

    bench.iter(|| unify(&expr_a, &expr_b));
}

#[bench]
fn nested_seq_with_late_unequal_word(bench: &mut Bencher) {
    let expr_a = expr!(seq(
        seq(
            wrd(a),
            wrd(k),
            wrd(b),
            seq(
                wrd(c),
                wrd(k),
                wrd(d),
                seq(wrd(e), wrd(k), wrd(f), seq(wrd(g), wrd(k), wrd(h)))
            )
        ),
        wrd(x)
    ));
    let expr_b = expr!(seq(
        seq(
            wrd(a),
            wrd(k),
            wrd(b),
            seq(
                wrd(c),
                wrd(k),
                wrd(d),
                seq(wrd(e), wrd(k), wrd(f), seq(wrd(g), wrd(k), wrd(h)))
            )
        ),
        wrd(y)
    ));

    bench.iter(|| unify(&expr_a, &expr_b));
}
