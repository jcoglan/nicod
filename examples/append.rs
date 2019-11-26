use nicod::expr::*;
use nicod::lang::*;
use nicod::*;
use std::rc::Rc;

fn derive(rules: &RuleSet, query: Rc<Expr>) {
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

    // [a, b, c] ++ [d, e] = ?

    derive(
        &rules,
        expr!(seq(
            lst(λ, [wrd(a), wrd(b), wrd(c),]),
            wrd(plus),
            lst(λ, [wrd(d), wrd(e),]),
            wrd(eq),
            var(answer)
        )),
    );

    // ? ++ ? = [a, b, c, d, e]

    derive(
        &rules,
        expr!(seq(
            var(x),
            wrd(plus),
            var(y),
            wrd(eq),
            lst(λ, [wrd(a), wrd(b), wrd(c), wrd(d), wrd(e),])
        )),
    );

    //  rev [] = []

    rules.insert(
        "rev-0",
        &expr!(seq(wrd(rev), lst(λ, []), wrd(eq), lst(λ, []))),
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
            wrd(eq),
            var(rev)
        )),
        &[
            expr!(seq(wrd(rev), var(tail), wrd(eq), var(rest))),
            expr!(seq(
                var(rest),
                wrd(plus),
                lst(λ, [var(head),]),
                wrd(eq),
                var(rev)
            )),
        ],
    );

    // rev [a, b, c] = ?

    let query = expr!(seq(
        wrd(rev),
        lst(λ, [wrd(a), wrd(b), wrd(c),]),
        wrd(eq),
        var(answer)
    ));

    derive(&rules, query.clone());
    for (_, proof) in rules.derive(&query) {
        println!("{}", proof);
    }

    //  [] : List

    rules.insert("type-0", &expr!(seq(lst(λ, []), wrd(is), wrd(List))), &[]);

    //       $tail : List
    //  ----------------------
    //  [$head | $tail] : List

    rules.insert(
        "type-N",
        &expr!(seq(lst(λ, [var(head) | var(tail)]), wrd(is), wrd(List))),
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

    // [a, b, c] : ?

    derive(
        &rules,
        expr!(seq(lst(λ, [wrd(a), wrd(b), wrd(c),]), wrd(is), var(answer))),
    );

    // ([a, b, c] ++ [d, e]) : ?

    let query = expr!(seq(
        seq(
            lst(λ, [wrd(a), wrd(b), wrd(c),]),
            wrd(plus),
            lst(λ, [wrd(d), wrd(e),])
        ),
        wrd(is),
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
                wrd(plus),
                seq(lst(λ, [wrd(b),]), wrd(plus), lst(λ, [wrd(c),]))
            ),
            wrd(plus),
            lst(λ, [wrd(d),])
        ),
        wrd(is),
        var(answer)
    ));

    derive(&rules, query.clone());
    for (_, proof) in rules.derive(&query) {
        println!("{}", proof);
    }
}
