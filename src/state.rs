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
        let x = self.resolve(x);
        let y = self.resolve(y);

        if x == y {
            return Some(self.clone());
        }

        match (&*x, &*y) {
            (Expr::Var(v), _) => Some(self.assign(v, &y)),
            (_, Expr::Var(v)) => Some(self.assign(v, &x)),
            (Expr::Seq(a), Expr::Seq(b)) => self.unify_seq(&a, &b),
            _ => None,
        }
    }

    fn unify_seq(&self, a: &Sequence, b: &Sequence) -> Option<State> {
        if a.0.len() != b.0.len() {
            return None;
        }

        let zip = a.0.iter().zip(&b.0);

        zip.fold(Some(self.clone()), |state, (x, y)| {
            state.and_then(|s| s.unify(x, y))
        })
    }

    fn assign(&self, var: &Variable, expr: &Rc<Expr>) -> State {
        let mut values = self.values.clone();
        values.insert(Rc::new(var.clone()), Rc::clone(expr));
        State { values }
    }
}
