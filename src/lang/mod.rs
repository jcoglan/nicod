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

struct Rule {
    name: String,
    premises: Vec<Expr>,
    conclusion: Expr,
}

impl RuleSet {
    pub fn new() -> RuleSet {
        RuleSet::default()
    }

    pub fn insert(&mut self, name: &str, conclusion: Expr, premises: Vec<Expr>) {
        let rule = Rule {
            name: String::from(name),
            premises,
            conclusion,
        };

        self.rules.insert(String::from(name), rule);
    }

    pub fn derive<'r>(&'r self, target: &'r Expr) -> Interleave<(State<'r>, Rc<Proof<'r>>)> {
        self.derive_in_state(&State::new(), (0, target))
    }

    fn derive_in_state<'r>(
        &'r self,
        state: &State<'r>,
        target: (usize, &'r Expr),
    ) -> Interleave<(State<'r>, Rc<Proof<'r>>)> {
        let streams = self.rules.values().map(|rule| {
            let invocation = Invocation {
                rule_set: self,
                rule,
                scope: state.scope(),
            };
            invocation.apply(state, target)
        });

        Interleave::new(streams)
    }
}

struct Invocation<'r> {
    rule_set: &'r RuleSet,
    rule: &'r Rule,
    scope: usize,
}

type Stream<'r, T> = BoxIter<'r, (State<'r>, T)>;

impl<'r> Invocation<'r> {
    fn apply(&self, state: &State<'r>, target: (usize, &'r Expr)) -> Stream<'r, Rc<Proof<'r>>> {
        let rule_name = &self.rule.name;

        let conclusion = (self.scope, &self.rule.conclusion);
        let conclusion_state = state.unify(target, conclusion);
        let premise_states = self.match_premises(conclusion_state);

        let proof_states = premise_states.map(move |(state, proofs)| {
            let proof = Proof::new(rule_name, &state, proofs, conclusion);
            (state, Rc::new(proof))
        });

        Box::new(proof_states)
    }

    fn match_premises(&self, state: Option<State<'r>>) -> Stream<'r, Vector<Rc<Proof<'r>>>> {
        let rule_set = self.rule_set;

        let premises = self.rule.premises.iter().map(|expr| (self.scope, expr));
        let init_states = Box::new(state.into_iter().map(|s| (s, Vector::new())));

        premises.fold(init_states, |states, premise| {
            let streams = states.map(move |(state, proofs)| {
                let proof_states = rule_set.derive_in_state(&state, premise);
                proof_states.map(move |(state, proof)| (state, concat(&proofs, &proof)))
            });

            Box::new(Flatten::new(streams))
        })
    }
}

fn concat<T: Clone>(list: &Vector<T>, item: &T) -> Vector<T> {
    let mut list = list.clone();
    list.push_back(item.clone());
    list
}
