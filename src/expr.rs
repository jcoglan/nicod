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
