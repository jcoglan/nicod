#[cfg(test)]
mod tests;

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Var(Variable),
    Wrd(Word),
    Seq(Sequence),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Variable(String);

#[derive(Debug, Clone, PartialEq)]
pub struct Word(String);

#[derive(Debug, Clone, PartialEq)]
pub struct Sequence(Vec<Expr>);

#[derive(Clone, Default)]
pub struct State {
    values: HashMap<Variable, Expr>,
}

impl State {
    pub fn new() -> State {
        State::default()
    }

    pub fn resolve(&self, expr: &Expr) -> Expr {
        match expr {
            Expr::Var(var) => {
                if let Some(value) = self.values.get(var) {
                    self.resolve(value)
                } else {
                    Expr::Var(var.clone())
                }
            }
            Expr::Seq(seq) => {
                let items = seq.0.iter().map(|item| self.resolve(item));
                Expr::Seq(Sequence(items.collect()))
            }
            _ => expr.clone(),
        }
    }

    pub fn unify(&self, x: &Expr, y: &Expr) -> Option<State> {
        let x = self.resolve(x);
        let y = self.resolve(y);

        if x == y {
            return Some(self.clone());
        }

        match (&x, &y) {
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

    fn assign(&self, var: &Variable, expr: &Expr) -> State {
        let mut values = self.values.clone();
        values.insert(var.clone(), expr.clone());
        State { values }
    }
}
