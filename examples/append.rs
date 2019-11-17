use nicod::expr::*;
use nicod::lang::*;
use nicod::proof;
use nicod::*;
use std::rc::Rc;

fn derive(rules: &RuleSet, query: Rc<Expr>) {
    println!("----[ {:?} ]----", query);

    for (i, (state, _)) in rules.derive(&query).enumerate() {
        println!("#{}: {:?}", i + 1, state.resolve(&query));
    }
    println!("");
}

fn main() {
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

    // (a (b (c nil))) ++ (d (e nil)) = ?

    derive(
        &rules,
        expr!(seq(
            seq(wrd(a), seq(wrd(b), seq(wrd(c), wrd(nil)))),
            wrd(plus),
            seq(wrd(d), seq(wrd(e), wrd(nil))),
            wrd(eq),
            var(answer)
        )),
    );

    // ? ++ ? = (a (b (c (d (e nil)))))
    let query = expr!(seq(
        var(x),
        wrd(plus),
        var(y),
        wrd(eq),
        seq(
            wrd(a),
            seq(wrd(b), seq(wrd(c), seq(wrd(d), seq(wrd(e), wrd(nil)))))
        )
    ));

    derive(&rules, query.clone());
    for (_, proof) in rules.derive(&query) {
        proof::display(&proof);
    }

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

    derive(
        &rules,
        expr!(seq(
            seq(wrd(a), seq(wrd(b), seq(wrd(c), wrd(nil)))),
            wrd(is),
            var(answer)
        )),
    );

    let query = expr!(seq(
        seq(
            seq(wrd(a), seq(wrd(b), seq(wrd(c), wrd(nil)))),
            wrd(plus),
            seq(wrd(d), seq(wrd(e), wrd(nil)))
        ),
        wrd(is),
        var(answer)
    ));

    derive(&rules, query.clone());
    for (_, proof) in rules.derive(&query) {
        proof::display(&proof);
    }
}
