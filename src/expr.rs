use std::fmt;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub enum Expr {
    Var(Variable),
    Wrd(Word),
    Seq(Sequence),
    Lst(List),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Variable(pub Rc<String>);

#[derive(Debug, PartialEq)]
pub struct Word(pub String);

#[derive(Debug, PartialEq)]
pub struct Sequence(pub Vec<Rc<Expr>>);

#[derive(Debug, PartialEq)]
pub struct List {
    pub tag: Rc<String>,
    pub pair: Option<Pair>,
}

#[derive(Debug, PartialEq)]
pub struct Pair {
    pub head: Rc<Expr>,
    pub tail: Rc<Expr>,
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Var(var) => var.fmt(f),
            Expr::Wrd(wrd) => wrd.fmt(f),
            Expr::Seq(seq) => seq.fmt(f),
            Expr::Lst(lst) => lst.fmt(f),
        }
    }
}

impl Expr {
    fn with_parens(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Seq(seq) => seq.with_parens(f),
            _ => fmt::Display::fmt(self, f),
        }
    }

    fn list_tail(&self, f: &mut fmt::Formatter, tag: &str) -> fmt::Result {
        match self {
            Expr::Lst(lst) if tag == *lst.tag => lst.list_head(f, false),
            _ => {
                write!(f, " | ")?;
                fmt::Display::fmt(self, f)
            }
        }
    }
}

impl fmt::Display for Sequence {
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

impl Sequence {
    fn with_parens(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(")?;
        fmt::Display::fmt(self, f)?;
        write!(f, ")")
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}[", self.tag)?;
        self.list_head(f, true)?;
        write!(f, "]")
    }
}

impl List {
    fn list_head(&self, f: &mut fmt::Formatter, is_first: bool) -> fmt::Result {
        if let Some(Pair { head, tail }) = &self.pair {
            if !is_first {
                write!(f, ", ")?;
            }
            fmt::Display::fmt(head, f)?;
            tail.list_tail(f, &self.tag)?;
        }
        Ok(())
    }
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "${}", self.0)
    }
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
