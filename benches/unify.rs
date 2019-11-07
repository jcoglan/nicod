#![feature(test)]
extern crate test;
use test::Bencher;

use nicod::*;

#[bench]
fn words(bench: &mut Bencher) {
    bench.iter(|| unify!(wrd(a), wrd(a)));
}

#[bench]
fn var(bench: &mut Bencher) {
    bench.iter(|| unify!(wrd(a), var(x)));
}

#[bench]
fn seqs_with_vars(bench: &mut Bencher) {
    bench.iter(|| {
        unify!(
            seq(wrd(a), var(w), wrd(b), var(x)),
            seq(var(y), wrd(c), var(z), wrd(d))
        )
    });
}

#[bench]
fn nested_seqs(bench: &mut Bencher) {
    bench.iter(|| {
        unify!(
            seq(
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
            ),
            seq(
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
            )
        )
    });
}

#[bench]
fn nested_seqs_with_indirect_vars(bench: &mut Bencher) {
    bench.iter(|| {
        unify!(
            seq(
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
            ),
            seq(
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
            )
        )
    });
}
