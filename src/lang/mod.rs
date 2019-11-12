mod tests;

use crate::expr::*;
use crate::iter::Interleave;
use crate::state::State;
use indexmap::map::IndexMap;
use std::rc::Rc;

type Stream<'a> = Box<dyn Iterator<Item = State> + 'a>;

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
            premises: Vec::from(premises),
            conclusion: Rc::clone(conclusion),
        };

        self.rules.insert(String::from(name), rule);
    }

    pub fn derive<'a>(&'a self, target: &Rc<Expr>) -> Stream<'a> {
        self.derive_in_state(&State::new(), target)
    }

    fn derive_in_state<'a>(&'a self, state: &State, target: &Rc<Expr>) -> Stream<'a> {
        let rules = self.rules.values();
        let streams = rules.map(|rule| rule.match_target(self, state, target));

        Box::new(Interleave::new(streams))
    }
}

struct Rule {
    premises: Vec<Rc<Expr>>,
    conclusion: Rc<Expr>,
}

impl Rule {
    fn match_target<'a>(
        &self,
        rule_set: &'a RuleSet,
        state: &State,
        target: &Rc<Expr>,
    ) -> Stream<'a> {
        let scope = state.scope();
        let premises = self.premises.iter().map(|premise| in_scope(scope, premise));
        let conclusion = in_scope(scope, &self.conclusion);

        let states = Box::new(state.unify(target, &conclusion).into_iter());

        premises.fold(states, |states, premise| {
            Box::new(states.flat_map(move |state| rule_set.derive_in_state(&state, &premise)))
        })
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
