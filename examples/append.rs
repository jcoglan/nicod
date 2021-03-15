use nicod::expr::*;
use nicod::lang::*;
use nicod::*;

fn derive(rules: &RuleSet, query: Expr) {
    println!("----[ {} ]----", query);

    for (i, (state, _)) in rules.derive(&query).enumerate() {
        println!("#{}: {}", i + 1, state.resolve(&query));
    }
    println!("");
}

fn main() {
    let mut rules = RuleSet::new();

    //  [] ++ $list = $list

    rules.insert(
        "append-0",
        &expr!(seq(lst(λ, []), wrd(+), var(list), wrd(=), var(list))),
        &[],
    );

    //            $tail ++ $list = $rest
    //  ------------------------------------------
    //  [$head | $tail] ++ $list = [$head | $rest]

    rules.insert(
        "append-N",
        &expr!(seq(
            lst(λ, [var(head) | var(tail)]),
            wrd(+),
            var(list),
            wrd(=),
            lst(λ, [var(head) | var(rest)])
        )),
        &[expr!(seq(
            var(tail),
            wrd(+),
            var(list),
            wrd(=),
            var(rest)
        ))],
    );

    // [a, b, c] ++ [d, e] = ?

    derive(
        &rules,
        expr!(seq(
            lst(λ, [wrd(a), wrd(b), wrd(c),]),
            wrd(+),
            lst(λ, [wrd(d), wrd(e),]),
            wrd(=),
            var(answer)
        )),
    );

    // ? ++ ? = [a, b, c, d, e]

    derive(
        &rules,
        expr!(seq(
            var(x),
            wrd(+),
            var(y),
            wrd(=),
            lst(λ, [wrd(a), wrd(b), wrd(c), wrd(d), wrd(e),])
        )),
    );

    //  rev [] = []

    rules.insert(
        "rev-0",
        &expr!(seq(wrd(rev), lst(λ, []), wrd(=), lst(λ, []))),
        &[],
    );

    //  rev $tail = $rest       $rest ++ [$head] = $rev
    //  -----------------------------------------------
    //            rev [$head | $tail] = $rev

    rules.insert(
        "rev-N",
        &expr!(seq(
            wrd(rev),
            lst(λ, [var(head) | var(tail)]),
            wrd(=),
            var(rev)
        )),
        &[
            expr!(seq(wrd(rev), var(tail), wrd(=), var(rest))),
            expr!(seq(
                var(rest),
                wrd(+),
                lst(λ, [var(head),]),
                wrd(=),
                var(rev)
            )),
        ],
    );

    // rev [a, b, c] = ?

    let query = expr!(seq(
        wrd(rev),
        lst(λ, [wrd(a), wrd(b), wrd(c),]),
        wrd(=),
        var(answer)
    ));

    derive(&rules, query.clone());
    for (_, proof) in rules.derive(&query) {
        println!("{}", proof);
    }

    //  [] : List

    rules.insert("type-0", &expr!(seq(lst(λ, []), wrd(:), wrd(List))), &[]);

    //       $tail : List
    //  ----------------------
    //  [$head | $tail] : List

    rules.insert(
        "type-N",
        &expr!(seq(lst(λ, [var(head) | var(tail)]), wrd(:), wrd(List))),
        &[expr!(seq(var(tail), wrd(:), wrd(List)))],
    );

    //  $a : List       $b : List
    //  -------------------------
    //      ($a ++ $b) : List

    rules.insert(
        "type-append",
        &expr!(seq(seq(var(a), wrd(+), var(b)), wrd(:), wrd(List))),
        &[
            expr!(seq(var(a), wrd(:), wrd(List))),
            expr!(seq(var(b), wrd(:), wrd(List))),
        ],
    );

    // [a, b, c] : ?

    derive(
        &rules,
        expr!(seq(lst(λ, [wrd(a), wrd(b), wrd(c),]), wrd(:), var(answer))),
    );

    // ([a, b, c] ++ [d, e]) : ?

    let query = expr!(seq(
        seq(
            lst(λ, [wrd(a), wrd(b), wrd(c),]),
            wrd(+),
            lst(λ, [wrd(d), wrd(e),])
        ),
        wrd(:),
        var(answer)
    ));

    derive(&rules, query.clone());
    for (_, proof) in rules.derive(&query) {
        println!("{}", proof);
    }

    // (([a] ++ ([b] ++ [c])) ++ [d]) : ?

    let query = expr!(seq(
        seq(
            seq(
                lst(λ, [wrd(a),]),
                wrd(+),
                seq(lst(λ, [wrd(b),]), wrd(+), lst(λ, [wrd(c),]))
            ),
            wrd(+),
            lst(λ, [wrd(d),])
        ),
        wrd(:),
        var(answer)
    ));

    derive(&rules, query.clone());
    for (_, proof) in rules.derive(&query) {
        println!("{}", proof);
    }
}
