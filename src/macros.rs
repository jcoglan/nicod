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
    (lst([$( $elem:tt )*])) => {
        expr!(lst(__tag, [$( $elem )*]))
    };
    (lst($tag:ident, $items:tt)) => {
        std::rc::Rc::new(Expr::Lst(List {
            tag: String::from(stringify!($tag)),
            pair: expr!(@list_items $tag $items),
        }))
    };
    (@list_items $tag:ident []) => {
        None
    };
    (@list_items $tag:ident [$n:ident $a:tt $( $rest:tt )*]) => {
        Some(Pair {
            head: expr!($n $a),
            tail: expr!(@list_tail $tag $( $rest )*),
        })
    };
    (@list_tail $tag:ident , $( $rest:tt )*) => {
        expr!(lst($tag, [$( $rest )*]))
    };
    (@list_tail $tag:ident | $( $rest:tt )*) => {
        expr!($( $rest )*)
    };
}

#[macro_export]
macro_rules! unify {
    ($xn:ident $xa:tt, $yn:ident $ya:tt) => {
        State::new().unify(&expr!($xn $xa), &expr!($yn $ya))
    };
}
