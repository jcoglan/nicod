use crate::expr::*;
use im::hashmap::HashMap;
use std::rc::Rc;

#[derive(Clone, Default)]
pub struct State {
    pub values: HashMap<Rc<Variable>, Rc<Expr>>,
}

impl State {
    pub fn new() -> State {
        State::default()
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
            (Expr::Seq(a), Expr::Seq(b)) => self.unify_seq(&a, &b),
            _ => false,
        }
    }

    fn unify_seq(&mut self, a: &Sequence, b: &Sequence) -> bool {
        if a.0.len() != b.0.len() {
            return false;
        }

        let zip = a.0.iter().zip(&b.0);

        zip.fold(true, |state, (x, y)| state && self.unify_mut(x, y))
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
