use std::fmt;
use std::rc::Rc;

#[derive(Clone, PartialEq)]
pub enum Expr {
    Var(Variable),
    Wrd(Word),
    Seq(Sequence),
    Lst(List),
}

impl Expr {
    pub fn in_scope(expr: &Rc<Expr>, scope: usize) -> Rc<Expr> {
        match &**expr {
            Expr::Var(var) => Rc::new(Expr::Var(var.in_scope(scope))),
            Expr::Seq(seq) => Rc::new(Expr::Seq(seq.in_scope(scope))),
            Expr::Lst(lst) => Rc::new(Expr::Lst(lst.in_scope(scope))),
            _ => Rc::clone(expr),
        }
    }

    fn with_parens(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Seq(seq) => seq.with_parens(f),
            _ => fmt::Debug::fmt(self, f),
        }
    }

    fn list_tail(&self, f: &mut fmt::Formatter, tag: &str) -> fmt::Result {
        match self {
            Expr::Lst(lst) if tag == lst.tag => lst.list_head(f, false),
            _ => {
                write!(f, " | ")?;
                fmt::Debug::fmt(self, f)
            }
        }
    }
}

impl fmt::Debug for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Var(var) => var.fmt(f),
            Expr::Wrd(wrd) => wrd.fmt(f),
            Expr::Seq(seq) => seq.fmt(f),
            Expr::Lst(lst) => lst.fmt(f),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Variable {
    name: Rc<String>,
    scope: usize,
}

impl Variable {
    pub fn new(name: &str) -> Variable {
        Variable {
            name: Rc::new(name.to_string()),
            scope: 0,
        }
    }

    fn in_scope(&self, scope: usize) -> Variable {
        Variable {
            name: Rc::clone(&self.name),
            scope,
        }
    }
}

impl fmt::Debug for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "${}", self.name)?;
        if self.scope > 0 {
            write!(f, "/{}", self.scope)?;
        }
        Ok(())
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
    fn in_scope(&self, scope: usize) -> Sequence {
        let items = self.0.iter().map(|item| Expr::in_scope(item, scope));
        Sequence(items.collect())
    }

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

#[derive(Clone, PartialEq)]
pub struct Pair {
    pub head: Rc<Expr>,
    pub tail: Rc<Expr>,
}

impl Pair {
    pub fn in_scope(&self, scope: usize) -> Pair {
        Pair {
            head: Expr::in_scope(&self.head, scope),
            tail: Expr::in_scope(&self.tail, scope),
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct List {
    pub tag: String,
    pub pair: Option<Pair>,
}

impl List {
    pub fn in_scope(&self, scope: usize) -> List {
        List {
            tag: self.tag.clone(),
            pair: self.pair.as_ref().map(|p| p.in_scope(scope)),
        }
    }

    fn list_head(&self, f: &mut fmt::Formatter, is_first: bool) -> fmt::Result {
        if let Some(Pair { head, tail }) = &self.pair {
            if !is_first {
                write!(f, ", ")?;
            }
            fmt::Debug::fmt(head, f)?;
            tail.list_tail(f, &self.tag)?;
        }
        Ok(())
    }
}

impl fmt::Debug for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}[", self.tag)?;
        self.list_head(f, true)?;
        write!(f, "]")
    }
}
