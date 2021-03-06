mod tests;

use crate::expr::*;
use im::hashmap::HashMap;
use std::rc::Rc;

#[derive(Clone, Default)]
pub struct State {
    values: HashMap<(usize, Rc<Variable>), (usize, Expr)>,
}

impl State {
    pub fn new() -> State {
        State::default()
    }

    pub fn scope(&self) -> usize {
        self.values.len() + 1
    }

    pub fn resolve(&self, expr: &Expr) -> Expr {
        self.resolve_scoped(expr, 0)
    }

    pub fn resolve_scoped(&self, expr: &Expr, scope: usize) -> Expr {
        match expr {
            Expr::Var(var) => {
                if let Some((scope, value)) = self.values.get(&(scope, var.clone())) {
                    self.resolve_scoped(value, *scope)
                } else {
                    expr.clone()
                }
            }
            Expr::Seq(seq) => {
                let items = seq.0.iter().map(|item| self.resolve_scoped(item, scope));
                Expr::Seq(Rc::new(Sequence(items.collect())))
            }
            Expr::Lst(lst) => {
                let pair = lst.pair.as_ref().map(|Pair { head, tail }| Pair {
                    head: self.resolve_scoped(&head, scope),
                    tail: self.resolve_scoped(&tail, scope),
                });

                Expr::Lst(Rc::new(List {
                    tag: lst.tag.clone(),
                    pair,
                }))
            }
            _ => expr.clone(),
        }
    }

    pub fn unify(&self, x: (usize, &Expr), y: (usize, &Expr)) -> Option<State> {
        let mut state = self.clone();

        if state.unify_mut(x, y) {
            Some(state)
        } else {
            None
        }
    }

    fn unify_mut(&mut self, x: (usize, &Expr), y: (usize, &Expr)) -> bool {
        let (x_scope, x) = self.resolve_var(x);
        let (y_scope, y) = self.resolve_var(y);

        match (&x, &y) {
            (Expr::Wrd(a), Expr::Wrd(b)) => a == b,
            (Expr::Var(a), Expr::Var(b)) if a == b && x_scope == y_scope => true,
            (Expr::Var(v), _) => self.assign((x_scope, v), (y_scope, y)),
            (_, Expr::Var(v)) => self.assign((y_scope, v), (x_scope, x)),
            (Expr::Seq(a), Expr::Seq(b)) => self.unify_sequence((x_scope, &a), (y_scope, &b)),
            (Expr::Lst(a), Expr::Lst(b)) => self.unify_list((x_scope, &a), (y_scope, &b)),
            _ => false,
        }
    }

    fn unify_sequence(&mut self, a: (usize, &Sequence), b: (usize, &Sequence)) -> bool {
        let (a_scope, Sequence(a_items)) = a;
        let (b_scope, Sequence(b_items)) = b;

        if a_items.len() != b_items.len() {
            return false;
        }

        a_items.iter().zip(b_items).fold(true, |state, (x, y)| {
            state && self.unify_mut((a_scope, x), (b_scope, y))
        })
    }

    fn unify_list(&mut self, a: (usize, &List), b: (usize, &List)) -> bool {
        let (a_scope, a) = a;
        let (b_scope, b) = b;

        if a.tag != b.tag {
            return false;
        }

        match (&a.pair, &b.pair) {
            (Some(a_pair), Some(b_pair)) => {
                self.unify_mut((a_scope, &a_pair.head), (b_scope, &b_pair.head))
                    && self.unify_mut((a_scope, &a_pair.tail), (b_scope, &b_pair.tail))
            }
            (None, None) => true,
            _ => false,
        }
    }

    fn assign(&mut self, var: (usize, &Rc<Variable>), expr: (usize, Expr)) -> bool {
        self.values.insert((var.0, var.1.clone()), expr);
        true
    }

    fn resolve_var(&self, expr: (usize, &Expr)) -> (usize, Expr) {
        let mut expr = expr;

        while let Expr::Var(var) = expr.1 {
            if let Some((scope, value)) = self.values.get(&(expr.0, var.clone())) {
                expr = (*scope, value);
            } else {
                break;
            }
        }

        (expr.0, expr.1.clone())
    }
}
