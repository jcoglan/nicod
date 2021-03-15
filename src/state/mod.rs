mod tests;

use crate::expr::*;
use im::hashmap::HashMap;
use std::rc::Rc;

#[derive(Clone, Default)]
pub struct State {
    values: HashMap<Rc<Variable>, Rc<Expr>>,
}

impl State {
    pub fn new() -> State {
        State::default()
    }

    pub fn scope(&self) -> usize {
        self.values.len() + 1
    }

    pub fn resolve(&self, expr: &Rc<Expr>) -> Rc<Expr> {
        match &**expr {
            Expr::Var(var) => {
                if let Some(value) = self.values.get(var) {
                    self.resolve(value)
                } else {
                    Rc::clone(expr)
                }
            }
            Expr::Seq(seq) => {
                let items = seq.0.iter().map(|item| self.resolve(item));
                Rc::new(Expr::Seq(Sequence(items.collect())))
            }
            Expr::Lst(lst) => {
                let pair = lst.pair.as_ref().map(|Pair { head, tail }| Pair {
                    head: self.resolve(&head),
                    tail: self.resolve(&tail),
                });
                Rc::new(Expr::Lst(List {
                    tag: Rc::clone(&lst.tag),
                    pair,
                }))
            }
            _ => Rc::clone(expr),
        }
    }

    pub fn unify(&self, x: &Rc<Expr>, y: &Rc<Expr>) -> Option<State> {
        let mut state = self.clone();

        if state.unify_mut(x, y) {
            Some(state)
        } else {
            None
        }
    }

    fn unify_mut(&mut self, x: &Rc<Expr>, y: &Rc<Expr>) -> bool {
        let x = self.resolve_var(x);
        let y = self.resolve_var(y);

        if x == y {
            return true;
        }

        match (&*x, &*y) {
            (Expr::Var(v), _) => self.assign(v, &y),
            (_, Expr::Var(v)) => self.assign(v, &x),
            (Expr::Seq(a), Expr::Seq(b)) => self.unify_sequence(&a, &b),
            (Expr::Lst(a), Expr::Lst(b)) => self.unify_list(&a, &b),
            _ => false,
        }
    }

    fn unify_sequence(&mut self, a: &Sequence, b: &Sequence) -> bool {
        if a.0.len() != b.0.len() {
            return false;
        }

        let zip = a.0.iter().zip(&b.0);
        zip.fold(true, |state, (x, y)| state && self.unify_mut(x, y))
    }

    fn unify_list(&mut self, a: &List, b: &List) -> bool {
        if a.tag != b.tag {
            return false;
        }

        match (&a.pair, &b.pair) {
            (Some(a_pair), Some(b_pair)) => {
                self.unify_mut(&a_pair.head, &b_pair.head)
                    && self.unify_mut(&a_pair.tail, &b_pair.tail)
            }
            (None, None) => true,
            _ => false,
        }
    }

    fn assign(&mut self, var: &Variable, expr: &Rc<Expr>) -> bool {
        self.values.insert(Rc::new(var.clone()), Rc::clone(expr));
        true
    }

    fn resolve_var(&self, expr: &Rc<Expr>) -> Rc<Expr> {
        let mut expr = expr;

        while let Expr::Var(var) = &**expr {
            if let Some(value) = self.values.get(var) {
                expr = value;
            } else {
                break;
            }
        }

        Rc::clone(expr)
    }
}
