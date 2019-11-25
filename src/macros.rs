#[macro_export]
macro_rules! var {
    ($n:ident) => {
        Variable::new(stringify!($n))
    };
}

#[macro_export]
macro_rules! expr {
    (var($x:ident)) => {
        std::rc::Rc::new(Expr::Var(var!($x)))
    };
    (wrd($x:ident)) => {
        std::rc::Rc::new(Expr::Wrd(Word(String::from(stringify!($x)))))
    };
    (seq($( $n:ident $a:tt ),+)) => {
        std::rc::Rc::new(Expr::Seq(Sequence(vec![$( expr!($n $a) ),+])))
    };
}

#[macro_export]
macro_rules! unify {
    ($xn:ident $xa:tt, $yn:ident $ya:tt) => {
        State::new().unify(&expr!($xn $xa), &expr!($yn $ya))
    };
}
