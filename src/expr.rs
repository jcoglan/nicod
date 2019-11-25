use std::fmt;
use std::rc::Rc;

#[derive(Clone, PartialEq)]
pub enum Expr {
    Var(Variable),
    Wrd(Word),
    Seq(Sequence),
}

impl Expr {
    fn with_parens(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Seq(seq) => seq.with_parens(f),
            _ => fmt::Debug::fmt(self, f),
        }
    }
}

impl fmt::Debug for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Var(var) => var.fmt(f),
            Expr::Wrd(wrd) => wrd.fmt(f),
            Expr::Seq(seq) => seq.fmt(f),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Variable(pub String);

impl fmt::Debug for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "${}", self.0)
    }
}

#[derive(Clone, PartialEq)]
pub struct Word(pub String);

impl fmt::Debug for Word {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, PartialEq)]
pub struct Sequence(pub Vec<Rc<Expr>>);

impl Sequence {
    fn with_parens(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(")?;
        fmt::Debug::fmt(self, f)?;
        write!(f, ")")
    }
}

impl fmt::Debug for Sequence {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, expr) in self.0.iter().enumerate() {
            expr.with_parens(f)?;
            if i < self.0.len() - 1 {
                write!(f, " ")?;
            }
        }
        Ok(())
    }
}
