mod tests;

use crate::expr::*;
use crate::iter::{BoxIter, Flatten, Interleave};
use crate::proof::Proof;
use crate::state::State;
use im::vector::Vector;
use indexmap::map::IndexMap;
use std::rc::Rc;

#[derive(Default)]
pub struct RuleSet {
    rules: IndexMap<String, Rule>,
}

impl RuleSet {
    pub fn new() -> RuleSet {
        RuleSet::default()
    }

    pub fn insert(&mut self, name: &str, conclusion: &Rc<Expr>, premises: &[Rc<Expr>]) {
        let rule = Rule {
            name: String::from(name),
            premises: Vec::from(premises),
            conclusion: Rc::clone(conclusion),
        };

        self.rules.insert(String::from(name), rule);
    }

    pub fn derive(&self, target: &Rc<Expr>) -> Interleave<(State, Rc<Proof>)> {
        self.derive_in_state(&State::new(), target)
    }

    fn derive_in_state(&self, state: &State, target: &Rc<Expr>) -> Interleave<(State, Rc<Proof>)> {
        let rules = self.rules.values();
        let streams = rules.map(|rule| rule.match_target(self, state, target));

        Interleave::new(streams)
    }
}

type Stream<'a, T> = BoxIter<'a, (State, T)>;

struct Rule {
    name: String,
    premises: Vec<Rc<Expr>>,
    conclusion: Rc<Expr>,
}

impl Rule {
    fn match_target<'a>(
        &'a self,
        rule_set: &'a RuleSet,
        state: &State,
        target: &Rc<Expr>,
    ) -> Stream<'a, Rc<Proof>> {
        let scope = state.scope();
        let premises = self.premises.iter().map(|premise| in_scope(scope, premise));
        let conclusion = in_scope(scope, &self.conclusion);

        let state_or_none = state.unify(target, &conclusion).into_iter();
        let init = Box::new(state_or_none.map(|state| (state, Vector::new())));

        let states: Stream<Vector<_>> = premises.fold(init, |states, premise| {
            let streams = states.map(move |(state, proofs)| {
                let proof_states = rule_set.derive_in_state(&state, &premise);
                proof_states.map(move |(state, proof)| (state, concat(&proofs, &proof)))
            });

            Box::new(Flatten::new(streams))
        });

        Box::new(states.map(move |(state, proofs)| {
            let proof = Proof::new(&self.name, &state, proofs, &conclusion);
            (state, Rc::new(proof))
        }))
    }
}

fn in_scope(scope: usize, expr: &Rc<Expr>) -> Rc<Expr> {
    match &**expr {
        Expr::Var(var) => {
            let name = format!("{}/{}", var.0, scope);
            Rc::new(Expr::Var(Variable(name)))
        }
        Expr::Seq(seq) => {
            let items = seq.0.iter().map(|item| in_scope(scope, item));
            Rc::new(Expr::Seq(Sequence(items.collect())))
        }
        _ => Rc::clone(expr),
    }
}

fn concat<T: Clone>(list: &Vector<T>, item: &T) -> Vector<T> {
    let mut list = list.clone();
    list.push_back(item.clone());
    list
}
