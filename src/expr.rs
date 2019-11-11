// use std::iter::{FromIterator, IntoIterator};
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Var(Variable),
    Wrd(Word),
    Seq(Sequence),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Variable(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct Word(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct Sequence(pub Vec<Rc<Expr>>);

impl Sequence {
    pub fn new(items: &[Rc<Expr>]) -> Sequence {
        Sequence(Vec::from(items))
    }

    pub fn type_eq(&self, other: &Sequence) -> bool {
        let mut zip = self.0.iter().zip(&other.0);

        self.0.len() == other.0.len()
            && zip.all(|(x, y)| match (&**x, &**y) {
                (Expr::Var(_), _) => true,
                (_, Expr::Var(_)) => true,
                (Expr::Wrd(a), Expr::Wrd(b)) => a == b,
                (Expr::Seq(a), Expr::Seq(b)) => a.0.len() == b.0.len(),
                _ => false,
            })
    }
}

/*

impl FromIterator<Rc<Expr>> for Sequence {
    fn from_iter<T>(iter: T) -> Sequence
    where
        T: IntoIterator<Item = Rc<Expr>>,
    {
        let mut types = Vec::new();
        let mut items = Vec::new();

        for item in iter {
            types.push(Type::from(&item));
            items.push(item);
        }
        Sequence(items, types)
    }
}

impl PartialEq for Sequence {
    fn eq(&self, other: &Sequence) -> bool {
        self.0 == other.0
    }
}

#[derive(Debug, Clone)]
enum Type {
    Var,
    Wrd(Rc<Expr>),
    Seq(usize),
}

impl Type {
    fn can_unify(a: &Vec<Type>, b: &Vec<Type>) -> bool {
        a.len() == b.len()
            && a.iter().zip(b).all(|(x, y)| match (x, y) {
                (Type::Var, _) => true,
                (_, Type::Var) => true,
                (Type::Wrd(p), Type::Wrd(q)) => p == q,
                (Type::Seq(p), Type::Seq(q)) => p == q,
                _ => false,
            })
    }
}

impl From<&Rc<Expr>> for Type {
    fn from(expr: &Rc<Expr>) -> Type {
        match &**expr {
            Expr::Var(_) => Type::Var,
            Expr::Wrd(_) => Type::Wrd(Rc::clone(expr)),
            Expr::Seq(seq) => Type::Seq(seq.0.len()),
        }
    }
}

*/
