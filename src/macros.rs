#[macro_export]
macro_rules! var {
    ($n:ident) => {
        Variable(String::from(stringify!($n)))
    };
}

#[macro_export]
macro_rules! expr {
    (var($x:ident)) => {
        Expr::Var(var!($x))
    };
    (wrd($x:ident)) => {
        Expr::Wrd(Word(String::from(stringify!($x))))
    };
    (seq($( $n:ident $a:tt ),+)) => {
        Expr::Seq(Sequence(vec![$( expr!($n $a) ),+]))
    };
}

#[macro_export]
macro_rules! unify {
    ($xn:ident $xa:tt, $yn:ident $ya:tt) => {
        State::new().unify(&expr!($xn $xa), &expr!($yn $ya))
    };
}
